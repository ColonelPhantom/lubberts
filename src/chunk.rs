// Parse Lua bytecode chunks

#[derive(Debug)]
pub enum ParseError {
    InvalidMagic,
    WrongVersion,
    WrongEndianness,
    WrongIntegerSize,
    WrongSizeT,
    WrongRawInstrSize,
    WrongLuaNumber,
}

#[derive(Debug)]
pub struct Chunk  {
}
impl Chunk {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ParseError> {
        // Begin by parsing the header
        if bytes[0..4] != [0x1B, 0x4C, 0x75, 0x61] {
            // 1B4C7561 aka (AsciiESC)Lua is the magic number at the start of every Lua bc chunk
            return Err(ParseError::InvalidMagic);
        }
        if bytes[4] != 0x51 || bytes[5] != 0x0 {
            // Bytecode must be for official Lua 5.1
            return Err(ParseError::WrongVersion);
        }
        if bytes[6] != 1 {
            // Only support little-endian for now
            return Err(ParseError::WrongEndianness);
        }
        if bytes[7] as usize != std::mem::size_of::<crate::Integer>() {
            return Err(ParseError::WrongIntegerSize);
        }
        if bytes[8] as usize != std::mem::size_of::<crate::SizeT>() {
            return Err(ParseError::WrongSizeT);
        }
        if bytes[9] as usize != std::mem::size_of::<crate::RawInstruction>() {
            return Err(ParseError::WrongRawInstrSize);
        }
        if bytes[10] as usize != std::mem::size_of::<crate::LuaNumber>()
            || bytes[11] != crate::NUMBER_IS_INTEGRAL as u8
        {
            return Err(ParseError::WrongLuaNumber);
        }

        // Header found to be valid
        let mut c = Chunk {};
        return Ok(c);
    }
}
        