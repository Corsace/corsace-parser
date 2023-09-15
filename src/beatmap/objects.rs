use itertools::Itertools;
use serde::{Deserialize, Serialize};
use tsify::Tsify;

pub enum HitWindows
{
    Hit50  = 199,
    Hit100 = 139,
    Hit300 = 79,
}

impl From<HitWindows> for f32
{
    fn from(value: HitWindows) -> Self { (value as i32) as f32 + 0.5 }
}

impl HitWindows
{
    pub fn with_od(self, od: f32) -> f32 { f32::from(self) - (od * 10.0) }
}
#[derive(Default, Serialize, Deserialize, Debug, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Pos2
{
    pub x: f32,
    pub y: f32,
}
#[derive(Default, Serialize, Deserialize, Debug, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct HitObject
{
    pub pos:        Pos2,
    pub start_time: f64,
    pub kind:       HitObjectKind,
}
#[derive(Default, Serialize, Deserialize, Debug, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum HitObjectKind
{
    #[default]
    Circle,
    Slider
    {
        pixel_len:      Option<f64>,
        repeats:        usize,
        control_points: Vec<PathControlPoint>,
        edge_sounds:    Vec<u8>,
    },
    Spinner
    {
        end_time: f64
    },
    Hold
    {
        end_time: f64
    },
}

#[derive(Default, Serialize, Deserialize, Debug, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct PathControlPoint
{
    pub pos:  Pos2,
    pub kind: Option<PathType>,
}

#[derive(Default, Serialize, Deserialize, Debug, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum PathType
{
    Catmull,
    Bezier,
    #[default]
    Linear,
    PerfectCurve,
}

impl From<rosu_pp::parse::Pos2> for Pos2
{
    fn from(value: rosu_pp::parse::Pos2) -> Self
    {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}
impl From<rosu_pp::parse::HitObject> for HitObject
{
    fn from(value: rosu_pp::parse::HitObject) -> Self
    {
        Self {
            pos:        value.pos.into(),
            start_time: value.start_time,
            kind:       value.kind.into(),
        }
    }
}
impl From<rosu_pp::parse::HitObjectKind> for HitObjectKind
{
    fn from(value: rosu_pp::parse::HitObjectKind) -> Self
    {
        match value
        {
            rosu_pp::parse::HitObjectKind::Circle => HitObjectKind::Circle,
            rosu_pp::parse::HitObjectKind::Slider {
                pixel_len,
                repeats,
                control_points,
                edge_sounds,
            } => HitObjectKind::Slider {
                pixel_len,
                repeats,
                control_points: control_points
                    .iter()
                    .map(|x| PathControlPoint::from(*x))
                    .collect_vec(),
                edge_sounds,
            },
            rosu_pp::parse::HitObjectKind::Spinner { end_time } =>
            {
                HitObjectKind::Spinner { end_time }
            }
            rosu_pp::parse::HitObjectKind::Hold { end_time } => HitObjectKind::Hold { end_time },
        }
    }
}
impl From<rosu_pp::parse::PathControlPoint> for PathControlPoint
{
    fn from(value: rosu_pp::parse::PathControlPoint) -> Self
    {
        Self {
            pos:  value.pos.into(),
            kind: value.kind.map(|x| x.into()),
        }
    }
}
impl From<rosu_pp::parse::PathType> for PathType
{
    fn from(value: rosu_pp::parse::PathType) -> Self
    {
        match value
        {
            rosu_pp::parse::PathType::Catmull => PathType::Catmull,
            rosu_pp::parse::PathType::Bezier => PathType::Bezier,
            rosu_pp::parse::PathType::Linear => PathType::Linear,
            rosu_pp::parse::PathType::PerfectCurve => PathType::PerfectCurve,
        }
    }
}
