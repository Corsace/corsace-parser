use std::io::Read;

use itertools::Itertools;
use libosu::prelude::Beatmap as libosuBeatmap;
use rosu_pp::{osu::OsuDifficultyAttributes, Beatmap, OsuPP};

use crate::replay::ParserResult;

use super::{objects::HitObject, Color, ParserBeatmap, ParserDifficulty};
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
    pub fn parse_extra(beatmap: &mut [u8]) -> ParserResult<Self>
    {
        let parsed = Beatmap::parse(&mut beatmap.as_ref())?;

        let diff_result = OsuPP::new(&parsed).calculate().difficulty;

        let mut res = ParserBeatmap::from(parsed);
        res.difficulty = Some(ParserDifficulty::from(diff_result));

        Ok(res)
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
            hit_objects: Some(
                value
                    .hit_objects
                    .iter()
                    .map(|x| HitObject::from(x.clone()))
                    .collect_vec(),
            ),
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
