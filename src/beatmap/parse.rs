use std::io::Read;

use itertools::Itertools;
use libosu::prelude::Beatmap as libosuBeatmap;
use rosu_pp::{
    osu::{OsuDifficultyAttributes, OsuGradualDifficultyAttributes, OsuPerformanceAttributes},
    Beatmap, OsuPP,
};

use crate::{replay::ParserResult, ParserScore};

use super::{
    objects::HitObject, Color, ParserBeatmap, ParserBeatmapAttributes, ParserDifficulty,
    ParserPerformance, ParserStrains,
};
impl ParserBeatmap
{
    pub fn parse<R: Read + Clone + std::convert::AsRef<[u8]>>(beatmap: &mut R)
        -> ParserResult<Self>
    {
        let parsed: ParserBeatmap = libosuBeatmap::parse(beatmap.as_ref())?.into();
        let mut parsed = parsed.extend_from_rosu(Beatmap::parse(beatmap.as_ref())?);
        parsed.hash = String::from(format!("{:x}", md5::compute(beatmap)));
        Ok(parsed)
    }
    pub fn parse_beatmap_strains<R: Read + Clone + std::convert::AsRef<[u8]>>(
        beatmap: &mut R, mods: Option<u32>,
    ) -> ParserResult<ParserStrains>
    {
        let map = Beatmap::parse(&mut beatmap.as_ref())?;
        let gradual_strains = OsuGradualDifficultyAttributes::new(&map, mods.unwrap_or(0))
            .map(|x| ParserDifficulty::from(x))
            .collect_vec();
        Ok(ParserStrains {
            difficulty:  Some(gradual_strains),
            performance: None,
        })
    }
    pub fn parse_extra<R: Read + Clone + std::convert::AsRef<[u8]>>(
        score: Option<ParserScore>, beatmap: &mut R,
    ) -> ParserResult<ParserBeatmapAttributes>
    {
        let mut map = Self::parse(&mut beatmap.as_ref())?;
        let rosu_map = Beatmap::parse(&mut beatmap.as_ref())?;

        let diff_result = match score
        {
            Some(score) =>
            {
                OsuPP::new(&rosu_map)
                    .mods(score.mods)
                    .combo(score.combo)
                    .n_misses(score.judgements.miss as _)
                    .accuracy(score.accuracy)
                    .calculate()
                    .difficulty
            }
            None => OsuPP::new(&rosu_map).calculate().difficulty,
        };
        let perf_result = match score
        {
            Some(score) => OsuPP::new(&rosu_map)
                .mods(score.mods)
                .combo(score.combo)
                .n_misses(score.judgements.miss as _)
                .accuracy(score.accuracy)
                .calculate(),
            None => OsuPP::new(&rosu_map).calculate(),
        };

        Ok(ParserBeatmapAttributes {
            difficulty:  Some(diff_result.into()),
            performance: Some(perf_result.into()),
        })
    }
    pub fn extend_from_rosu(self, value: Beatmap) -> Self
    {
        Self {
            circles: value.n_circles,
            sliders: value.n_sliders,
            spinners: value.n_spinners,
            ar: value.ar,
            od: value.od,
            cs: value.cs,
            hp: value.hp,
            slider_multiplier: value.slider_mult,
            tick_rate: value.tick_rate,
            difficulty: None,
            ..self
        }
    }
}

impl From<rosu_pp::Beatmap> for ParserBeatmap
{
    fn from(value: rosu_pp::Beatmap) -> Self
    {
        Self {
            circles: value.n_circles,
            sliders: value.n_sliders,
            spinners: value.n_spinners,
            ar: value.ar,
            od: value.od,
            cs: value.cs,
            hp: value.hp,
            slider_multiplier: value.slider_mult,
            tick_rate: value.tick_rate,
            difficulty: None,
            hit_objects: Some(
                value
                    .hit_objects
                    .iter()
                    .map(|x| HitObject::from(x.clone()))
                    .collect_vec(),
            ),

            ..Default::default()
        }
    }
}
impl From<OsuDifficultyAttributes> for ParserDifficulty
{
    fn from(value: OsuDifficultyAttributes) -> Self
    {
        Self {
            aim_strain:          value.aim,
            speed_strain:        value.speed,
            flashlight_strain:   value.flashlight,
            slider_strain_ratio: value.slider_factor,
            speed_note_count:    value.speed_note_count,
            ar:                  value.ar,
            od:                  value.od,
            hp:                  value.hp,
            circles:             value.n_circles as _,
            sliders:             value.n_sliders as _,
            spinners:            value.n_spinners as _,
            stars:               value.stars,
            max_combo:           value.max_combo as _,
        }
    }
}
impl From<OsuPerformanceAttributes> for ParserPerformance
{
    fn from(value: OsuPerformanceAttributes) -> Self
    {
        Self {
            difficulty:           value.difficulty.into(),
            pp:                   value.pp,
            pp_acc:               value.pp_acc,
            pp_aim:               value.pp_aim,
            pp_flashlight:        value.pp_flashlight,
            pp_speed:             value.pp_speed,
            effective_miss_count: value.effective_miss_count,
        }
    }
}
impl From<libosuBeatmap> for ParserBeatmap
{
    fn from(value: libosuBeatmap) -> Self
    {
        Self {
            title: value.title,
            artist: value.artist,
            tags: value.tags,
            combo_colors: value.colors.iter().map(|x| Color::from(*x)).collect_vec(),
            ..Default::default()
        }
    }
}
