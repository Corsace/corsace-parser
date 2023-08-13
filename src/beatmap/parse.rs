use std::io::Read;

use itertools::Itertools;
use libosu::prelude::Beatmap as libosuBeatmap;
use rosu_pp::{
    osu::{
        OsuDifficultyAttributes, OsuGradualDifficultyAttributes, OsuGradualPerformanceAttributes,
        OsuPerformanceAttributes, OsuScoreState,
    },
    Beatmap, OsuPP,
};

use crate::{console_log, replay::ParserResult, ParserScore};

use super::{
    objects::HitObject, Color, ParserBeatmap, ParserBeatmapAttributes, ParserDifficulty,
    ParserPerformance, ParserScoreState, ParserStrains,
};
impl ParserBeatmap
{
    pub fn parse<R: Read + Clone + std::convert::AsRef<[u8]>>(beatmap: &mut R)
        -> ParserResult<Self>
    {
        let parsed: ParserBeatmap = libosuBeatmap::parse(beatmap.as_ref())?.into();

        let mut rosu_map = Beatmap::parse(beatmap.as_ref())?;
        let mut parsed = parsed.extend_from_rosu(&rosu_map);

        parsed.max_combo = OsuPP::new(&rosu_map).calculate().difficulty.max_combo as u32;
        parsed.hash = String::from(format!("{:x}", md5::compute(beatmap)));

        Ok(parsed)
    }
    pub fn parse_beatmap_strains<R: Read + Clone + std::convert::AsRef<[u8]>>(
        beatmap: &mut R, score_states: Option<Vec<ParserScoreState>>, mods: Option<u32>,
    ) -> ParserResult<ParserStrains>
    {
        let map = Beatmap::parse(&mut beatmap.as_ref())?;

        let gradual_strains = OsuGradualDifficultyAttributes::new(&map, mods.unwrap_or(0))
            .map(|x| ParserDifficulty::from(x))
            .collect_vec();

        let gradual_perf = if let Some(states) = score_states
        {
            let mut perf = OsuGradualPerformanceAttributes::new(&map, mods.unwrap_or(0));
            let mut perf_results: Vec<ParserPerformance> = Vec::new();
            for state in states
            {
                let res = perf.process_next_object(state.into());
                if res.is_none()
                {
                    break;
                }
                perf_results.push(res.unwrap().into());
            }
            Some(perf_results)
        }
        else
        {
            None
        };

        Ok(ParserStrains {
            difficulty:  Some(gradual_strains),
            performance: gradual_perf,
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
    pub fn extend_from_rosu(self, value: &Beatmap) -> Self
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
            diff_name: value.difficulty_name,
            combo_colors: value.colors.iter().map(|x| Color::from(*x)).collect_vec(),
            ..Default::default()
        }
    }
}

impl From<ParserScoreState> for OsuScoreState
{
    fn from(value: ParserScoreState) -> Self
    {
        Self {
            max_combo: value.max_combo as _,
            n300:      value.n300 as _,
            n100:      value.n100 as _,
            n50:       value.n50 as _,
            n_misses:  value.n_misses as _,
        }
    }
}
