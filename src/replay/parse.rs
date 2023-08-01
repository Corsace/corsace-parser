use std::io::Read;

use super::{Judgements, LifegraphData, Mods, ParserResult, Replay};
use crate::{
    beatmap::ParserBeatmap,
    replay::{Mode, ParserError},
};
use byteorder::{LittleEndian, ReadBytesExt};
use rosu_pp::Beatmap;
use thiserror::Error;

pub type LEBResult<T, E = LEB128Error> = std::result::Result<T, E>;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum LEB128Error
{
    #[error("buffer overflowed")]
    BufferOverflow,
    #[error("LEB128 value overflowed")]
    LEB128Overflow,
    #[error("error decoding ULEB128 value")]
    ULEBDecodeError,
    #[error("error decoding ULEB128 string, invalid char {0}")]
    ULEBStringError(u8),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("string conversion error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
}

pub trait ULEB128Decode: Read
{
    const LEB128_HIGH_ORDER_BIT: u8 = 1 << 7;
    const LEB128_SIGN_BIT: u8 = 1 << 6;

    fn read_uleb128(&mut self) -> LEBResult<u64>
    {
        let mut result = 0;
        let mut shift = 0;
        loop
        {
            let byte = self.read_u8().map_err(|_| LEB128Error::BufferOverflow)?;
            if shift == 63 && byte > 1
            {
                return Err(LEB128Error::LEB128Overflow);
            }
            result |= u64::from(byte & !Self::LEB128_HIGH_ORDER_BIT) << shift;

            if byte & Self::LEB128_HIGH_ORDER_BIT == 0
            {
                return Ok(result);
            }

            shift += 7;
        }
    }

    fn read_uleb128_string(&mut self) -> LEBResult<String>
    {
        match self.read_u8()?
        {
            0x0 => Ok(String::new()),
            0xB =>
            {
                let len = self.read_uleb128()?;
                if len == 0
                {
                    return Ok(String::new());
                }
                let mut buf = vec![0; len as usize];
                self.read_exact(&mut buf)?;
                let str = String::from_utf8(buf)?;
                Ok(str)
            }
            x =>
            {
                return Err(LEB128Error::ULEBStringError(x));
            }
        }
    }
}

impl<R: Read + ?Sized> ULEB128Decode for R {}
impl Replay
{
    pub fn parse<R: Read>(replay: &mut R, extra: bool) -> ParserResult<Replay>
    {
        let mode = match replay.read_u8()?
        {
            0 => Mode::Osu,
            1 => Mode::Taiko,
            2 => Mode::Catch,
            3 => Mode::Mania,
            x => return Err(ParserError::InvalidMode(x)),
        };

        let version = replay.read_u32::<LittleEndian>()?;
        let beatmap_hash = replay.read_uleb128_string()?;
        let username = replay.read_uleb128_string()?;
        let replay_hash = replay.read_uleb128_string()?;
        let count_300 = replay.read_u16::<LittleEndian>()?;
        let count_100 = replay.read_u16::<LittleEndian>()?;
        let count_50 = replay.read_u16::<LittleEndian>()?;
        let count_geki = replay.read_u16::<LittleEndian>()?;
        let count_katu = replay.read_u16::<LittleEndian>()?;
        let miss = replay.read_u16::<LittleEndian>()?;
        let score = replay.read_u32::<LittleEndian>()?;
        let max_combo = replay.read_u16::<LittleEndian>()?;
        let perfect = replay.read_u8()? == 1;
        let mods = replay.read_u32::<LittleEndian>()?;

        let life_graph = replay
            .read_uleb128_string()?
            .split(",")
            .filter_map(|entry| {
                if entry.is_empty()
                {
                    None
                }
                else
                {
                    Some(entry.split("|"))
                }
            })
            .map(|mut entry| {
                let time = entry
                    .next()
                    .ok_or(ParserError::LifeGraphMissing)?
                    .parse::<i32>()?;
                let life_value = entry
                    .next()
                    .ok_or(ParserError::LifeGraphMissing)?
                    .parse::<f64>()?;

                Ok(LifegraphData { time, life_value })
            })
            .collect::<ParserResult<Vec<_>>>()?;

        let timestamp = replay.read_u64::<LittleEndian>()?;
        let replay_data_length = replay.read_u32::<LittleEndian>()?;
        let mut replay_data = None;
        if extra
        {
            replay_data = Some(vec![0; replay_data_length as usize]);
            replay.read_exact(&mut replay_data.as_mut().unwrap())?;
        }

        let score_id = match replay.read_u64::<LittleEndian>()?
        {
            0 => None,
            v => Some(v.to_string()),
        };

        Ok(Replay {
            mode,
            version,
            beatmap_hash,
            replay_hash,
            username,
            judgements: Judgements {
                count_300,
                count_100,
                count_50,
                count_geki,
                count_katu,
                miss,
            },
            score,
            max_combo,
            perfect,
            mods: Mods::from_bits(mods).ok_or(ParserError::UnexpectedMods(mods))?,
            life_graph,
            timestamp: timestamp.to_string(),
            replay_data: replay_data,
            score_id,
            ..Default::default()
        })
    }

    pub fn parse_extra<R: Read + Clone + std::convert::AsRef<[u8]>>(
        replay: &mut R, beatmap: &mut R,
    ) -> ParserResult<Replay>
    {
        let mut replay = Replay::parse(replay, true)?;
        let beatmap = ParserBeatmap::parse(beatmap)?;

        if replay.beatmap_hash != beatmap.hash
        {
            return Err(ParserError::BeatmapHashMismatch(
                replay.beatmap_hash,
                beatmap.hash,
            ));
        }

        let mut decoded_data = Vec::new();
        lzma_rs::lzma_decompress(
            &mut replay.replay_data.unwrap().as_slice(),
            &mut decoded_data,
        )?;
        let replay_data = String::from_utf8(decoded_data)?;

        let mut frames = replay_data
            .split(",")
            .filter(|entry| !entry.trim().is_empty())
            .map(|entry| {
                let mut data = entry.split("|");
                // time
                // x and y
                // button bits
            });

        todo!()
    }
}
