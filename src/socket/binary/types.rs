use std::io::{self, Write, Cursor, Read};
use byteorder::{WriteBytesExt, ReadBytesExt, BigEndian, LittleEndian};

pub struct RaknetWriter {
    buffer: Vec<u8>,
}

impl RaknetWriter {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    pub fn write_byte(&mut self, value: u8) {
        self.buffer.push(value);
    }

    pub fn write_long(&mut self, value: i64) {
        self.buffer.write_i64::<BigEndian>(value).unwrap();
    }

    pub fn write_magic(&mut self) {
        let magic: [u8; 16] = [0x00, 0xff, 0xff, 0x00, 0xfe, 0xfe, 0xfe, 0xfe, 0xfd, 0xfd, 0xfd, 0xfd, 0x12, 0x34, 0x56, 0x78];
        self.buffer.extend_from_slice(&magic);
    }

    pub fn write_short(&mut self, value: i16) {
        self.buffer.write_i16::<BigEndian>(value).unwrap();
    }

    pub fn write_unsigned_short(&mut self, value: u16) {
        self.buffer.write_u16::<BigEndian>(value).unwrap();
    }

    pub fn write_string(&mut self, value: &str) {
        let length = value.len() as u16;
        self.write_unsigned_short(length);
        self.buffer.extend_from_slice(value.as_bytes());
    }

    pub fn write_boolean(&mut self, value: bool) {
        self.buffer.push(if value { 1 } else { 0 });
    }

    pub fn write_int(&mut self, value: i32) {
        self.buffer.write_i32::<LittleEndian>(value).unwrap();
    }

    pub fn to_bytes(self) -> Vec<u8> {
        self.buffer
    }
}

pub struct RaknetReader<'a> {
    cursor: Cursor<&'a [u8]>,
}

impl<'a> RaknetReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { cursor: Cursor::new(data) }
    }

    pub fn read_byte(&mut self) -> io::Result<u8> {
        self.cursor.read_u8()
    }

    pub fn read_long(&mut self) -> io::Result<i64> {
        self.cursor.read_i64::<BigEndian>()
    }

    pub fn read_magic(&mut self) -> io::Result<[u8; 16]> {
        let mut magic = [0u8; 16];
        self.cursor.read_exact(&mut magic)?;
        Ok(magic)
    }

    pub fn read_short(&mut self) -> io::Result<i16> {
        self.cursor.read_i16::<BigEndian>()
    }

    pub fn read_unsigned_short(&mut self) -> io::Result<u16> {
        self.cursor.read_u16::<BigEndian>()
    }

    pub fn read_string(&mut self) -> io::Result<String> {
        let length = self.read_unsigned_short()? as usize;
        let mut buffer = vec![0u8; length];
        self.cursor.read_exact(&mut buffer)?;
        Ok(String::from_utf8(buffer).unwrap())
    }

    pub fn read_boolean(&mut self) -> io::Result<bool> {
        Ok(self.read_byte()? != 0)
    }

    pub fn read_int(&mut self) -> io::Result<i32> {
        self.cursor.read_i32::<LittleEndian>()
    }
}
