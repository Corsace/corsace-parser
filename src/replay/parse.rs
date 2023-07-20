use std::io::Read;

use byteorder::ReadBytesExt;

use super::{ParserResult, Replay};
use crate::replay::{Mode, ParserError};

impl Replay {
    pub fn parse<R: Read>(replay: &mut R) -> ParserResult<Replay> {
        let mode = match replay.read_u8()? {
            0 => Mode::Osu,
            1 => Mode::Taiko,
            2 => Mode::Catch,
            3 => Mode::Mania,
            x => return Err(ParserError::InvalidMode(x)),
        };

        todo!()
    }
}
