// Imports
use std::fs::{self, File};
use std::io::Read;
use hashbrown::HashMap;

// TODO: ngln ns, nall will always be positive so we can convert this 
// to an unsigned integer when it is ready
#[derive(Debug)]
struct Header {
    tgl: [f64; 4],
    ngl: i32,
    ns: i32,
    nall: i32,
    thdate: String
}

// Creates Enumerators to define type of Reactant
#[derive(Debug)]
enum ReacType {
    Gas,
    Condense,
    GasCondense,
}

// Structure to hold Data on Reactants form thermo.lib
#[derive(Debug)]
struct Reactant {
    react_type: ReacType,
    x: i32,
}

// Opens File to thermo.lib
pub fn read_header(file: &mut File) -> Result<(), Box<dyn std::error::Error>> {
    let record_size = super::read_record(file).expect("Could not read record");

    // Extraction of data from thermo.lib
    let mut data: Vec<u8> = vec![0; record_size as usize];
    file.read_exact(&mut data)?;
    
    let header = Header {
        tgl: [
            f64::from_le_bytes(data[0..8].try_into()?),
            f64::from_le_bytes(data[8..16].try_into()?),
            f64::from_le_bytes(data[16..24].try_into()?),
            f64::from_le_bytes(data[24..32].try_into()?),
        ],

        ngl: i32::from_le_bytes(data[32..36].try_into()?),
        ns: i32::from_le_bytes(data[36..40].try_into()?),
        nall: i32::from_le_bytes(data[40..44].try_into()?),

        thdate: String::from_utf8(data[44..54].try_into()?)?,
    };

    println!("Header Data: {header:?}");

    Ok(())

}

pub fn read_data() -> HashMap<String, Vec<f64>> {
    let test: HashMap<String, Vec<f64>> = HashMap::new();
    test

    // Layout for continuing next
    // Depending on ifaz we are going to pass the values to the decoder that we want
    // 0 -> read_gas()
    // 1 -> read_condense()
    // 2 -> read_gas_condense()
}

// Reading Gaseous Values from the .lib file
pub fn read_gas() -> f64 {
    let test = 54.0;
    test
}

// Reading condensed values from the .lib file
pub fn read_condense() -> f64 {
    let test = 69.0;
    test
}

// Reading Gaseous Condensed Values from the .lib file
pub fn read_gas_condense() -> f64 {
    let test = 89.0;
    test
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::files::*;
    use std::fs::File;

    #[test]
    fn test_header() {
        let path = "C:/Users/matth/OneDrive - purdue.edu/Outside/Rust/cea-rs/src/thermo.lib";
        let mut thermo = File::open(path).expect("Could not read file");
    
        let header = read_header(&mut thermo);
        println!("{header:?}");

        let record2 = read_record(&mut thermo).unwrap();
        println!("Next Record: {record2}");
    }
}