use std::fs::File;
use std::io::{self, Read};
use byteorder::{LittleEndian, BigEndian, ReadBytesExt};
use std::path::Path;



// Struct to hold header
#[derive(Debug)]
struct Header {
    tgl: [f64; 4],
    ngl: u32,
    ns: u32,
    nall: u32,
    thdate: String,
}

fn main() {
    // if cfg!(target_os = "windows") {
    //     println!("Running on Windows");
    // } else if cfg!(target_os = "linux") {

    // }



}

fn read_u32<R: Read>(reader: &mut R, endian: &str) -> io::Result<u32> {
    match endian {
        "<" => reader.read_u32::<LittleEndian>(),
        ">" => reader.read_u32::<BigEndian>(),
        _ => panic!("Invalid Endians"),
    }
}

fn read_i32<R: Read>(reader: &mut R, endian: &str) -> io::Result<i32> {
    match endian {
        "<" => reader.read_i32::<LittleEndian>(),
        ">" => reader.read_i32::<LittleEndian>(),
        _ => panic!("Invalid Endians"),
    }
}

fn read_f64<R: Read>(reader: &mut R, endian: &str) -> io::Result<f64> {
    match endian {
        "<" => reader.read_f64::<LittleEndian>(),
        ">" => reader.read_f64::<BigEndian>(),
        _ => panic!("Invalid Endians"),
    }
}

