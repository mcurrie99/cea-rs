use cea_rs;
use std::fs::File;

fn main() {
    // if cfg!(target_os = "windows") {
    //     println!("Running on Windows");
    // } else if cfg!(target_os = "linux") {

    // }

    // let x = read_f64(reader, endian)

    // TODO: Figure out logic to run legacy code
    cea_rs::legacy::run_legacy();

}

