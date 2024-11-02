use std::io::{self, Read};
use byteorder::{LittleEndian, BigEndian, ReadBytesExt};
use std::fs::File;

pub mod thermo;

pub fn read_record(file: &mut File) -> Result<i32, std::io::Error> {
    // Reads 4-byte Record from file
    let mut buffer: [u8; 4] = [0; 4];
    file.read_exact(&mut buffer)?;
    Ok(i32::from_le_bytes(buffer))
}

pub fn read_u32<R: Read>(reader: &mut R, endian: &str) -> io::Result<u32> {
    match endian {
        "<" => reader.read_u32::<LittleEndian>(),
        ">" => reader.read_u32::<BigEndian>(),
        _end => panic!("Invalid Endian Type: {_end}")
    }
}

pub fn read_i32<R: Read>(reader: &mut R, endian: &str) -> io::Result<i32> {
    match endian {
        "<" => reader.read_i32::<LittleEndian>(),
        ">" => reader.read_i32::<BigEndian>(),
        _end => panic!("Invalid Endian Type: {_end}")
    }
}

pub fn read_f64<R: Read>(reader: &mut R, endian: &str) -> io::Result<f64> {
    match endian {
        "<" => reader.read_f64::<LittleEndian>(),
        ">" => reader.read_f64::<BigEndian>(),
        _end => panic!("Invalid Endian Type: {_end}")
    }
}
