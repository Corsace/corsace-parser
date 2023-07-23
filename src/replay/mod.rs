pub mod parse;
pub mod utils;

use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use self::parse::LEB128Error;

pub type ParserResult<T, E = ParserError> = std::result::Result<T, E>;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum ParserError
{
    #[error("error creating lzma decoder: {0}")]
    LzmaCreate(#[from] lzma_rs::error::Error),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("error decoding utf8: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("error parsing int: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("error parsing float: {0}")]
    ParseFloat(#[from] std::num::ParseFloatError),

    #[error("error parsing string")]
    ParseString(#[from] crate::replay::LEB128Error),

    #[error("missing field in life graph")]
    LifeGraphMissing,

    #[error("unexpected mods: {0}")]
    UnexpectedMods(u32),

    #[error("invalid mode: {0}")]
    InvalidMode(u8),

    #[error("invalid buttons: {0}")]
    InvalidButtons(u32),
}
#[derive(Default, Serialize, Deserialize, Debug)]

pub enum Mode
{
    #[default]
    Osu   = 0,

    Taiko = 1,

    Catch = 2,

    Mania = 3,
}
#[derive(Default, Serialize, Deserialize, Debug)]

pub struct Judgements
{
    pub count_300:  u16,
    pub count_100:  u16,
    pub count_50:   u16,
    pub count_geki: u16,
    pub count_katu: u16,
    pub miss:       u16,
}
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct LifegraphData
{
    //time in ms
    pub time:       i32,
    pub life_value: f64,
}
mod integer_representation
{
    use serde::{self, Deserialize, Deserializer, Serialize, Serializer};

    // CHANGE THIS ACCORDING TO YOUR CODE
    use crate::replay::Mods;
    type IntRep = u32;
    type Flags = Mods;

    pub fn serialize<S>(date: &Flags, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        date.bits().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Flags, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw: IntRep = IntRep::deserialize(deserializer)?;
        Mods::from_bits(raw).ok_or(serde::de::Error::custom(format!(
            "Unexpected flags value {}",
            raw
        )))
    }
}

bitflags! {

    #[derive(Default, Serialize, Deserialize, Debug)]
    #[serde(transparent)]
    pub struct Mods: u32 {
        const None = 0;
        const NoFail = 1;
        const Easy = 2;
        const TouchDevice = 4;
        const Hidden = 8;
        const HardRock = 16;
        const SuddenDeath = 32;
        const DoubleTime = 64;
        const Relax = 128;
        const HalfTime = 256;
        const Nightcore = 512;
        const Flashlight = 1024;
        const Autoplay = 2048;
        const SpunOut = 4096;
        const Relax2 = 8192;
        const Perfect = 16384;
        const Key4 = 32768;
        const Key5 = 65536;
        const Key6 = 131072;
        const Key7 = 262144;
        const Key8 = 1015808;
        const FadeIn = 1048576;
        const Random = 2097152;
        const LastMod = 4194304;
        const TargetPractice = 8388608;
        const Key9 = 16777216;
        const Key10 = 33554432;
        const Key1 = 67108864;
        const Key3 = 134217728;
        const Key2 = 268435456;
    }
}
#[derive(Default, Serialize, Deserialize, Debug)]

pub struct Replay
{
    pub mode:         Mode,
    pub version:      u32,
    pub beatmap_hash: String,
    pub replay_hash:  String,
    pub username:     String,
    pub judgements:   Judgements,
    pub score:        u32,
    pub max_combo:    u16,
    pub perfect:      bool,
    #[serde(with = "integer_representation")]
    pub mods:         Mods,
    pub life_graph:   Vec<LifegraphData>,
    // measured in windows ticks
    pub timestamp:    u64,
    pub replay_data:  Vec<u8>,
    pub score_id:     Option<u64>,
}
