// NOTES FOR MATT DURING DEVELOPMENT
// - Fortran does not seem to have dynamically sizing arrays
//   Ensure to use caution since we have no check, we can implement a 
//   structure to do this but it would be unecessary.


// Support for legacy CEA will be added here so that the application
// Can be ran in a way that is compatible with old versions
use std::io::{self, Write, Read};
use std::fs::{File, OpenOptions};
use tempfile::tempfile;


// The following paramters set the maximum dimensions for many variables
//  They are definedi n part 2, pages 39 of the manuals, NASA RP-1311
//  The variable NCOL set the number of columns in the output. It may
//  be increased for wider paper or smaller fonts
const MAXNGC: i32 = 600;
const MAXNC: i32 = 300;
const NCOL: i32 = 8;
const MAXMAT: i32 = 50;
const MAXTR: i32 = 40;
const MAXR: i32 = 24;
const MAXEL: i32 = 20;
const MAXNG: i32 = 500;
const MAXMIX: i32 = 52;
const MAXT: i32 = 51;
const MAXPV: i32 = 26;

// The following parameters set the input/output unit numbers. These
//  numbers are also defined in the manual, part 2 p39, and may be
//  adjusted as desired
const IOINP: i32 = 7;
const IOOUT: i32 = 8;
const IOSCH: i32 = 13;
const IOTHM: i32 = 14;
const IOPLT: i32 = 15;
const IOTRN: i32 = 18;

// TODO: Ensure that these declarations are necessary
// We can probably remove a few from this and be fine
pub fn run_legacy() {
    // Local Variables Declarations
    // String Array
    let mut ensert: [String; 20] = Default::default();

    // Booleans
    let caseok: bool;
    let ex: bool;

    // Integers
    let i: i32;
    let inc: i32;
    let iof: i32;
    let j: i32;
    let ln: i32;
    let n: i32;
    let INDEX: i32;

    // Floats
    let xi: f64;
    let xln: f64;
    let DLOG: f64;

    // Prints to console
    println!(" ENTER INPUT FILE NAME WITHOUT .INP EXTENSION");
    println!("   THE OUTPUT FILES FOR LISTING AND PLOTTING WILL HAVE");
    println!(" THE SAME NAME WITH EXTENSIONS .out AND .plt RESPECTIVELY\n\n");

    // Grabs value from input and formats it.
    let mut prefix = String::new();
    io::stdin()
        .read_line(&mut prefix)
        .expect("failt to readline");
    prefix = String::from(prefix.trim());

    // TODO: this is just here to follow logic of fortran
    ln = prefix.len() as i32;

    // Creates variables for file names
    let infile = prefix.clone() + ".inp";
    let ofile = prefix.clone() + ".out";
    let pfile = prefix.clone() + ".plt";

    // Tries to open infile
    let err_msg = format!("{} DOES NOT EXIST", infile);
    let ioinp = File::open(&infile).expect(err_msg.as_str());

    // Opens or creates the outfile
    let mut ioout = match OpenOptions::new().read(true).write(true).open(&ofile) {
        Ok(file) => file,
        Err(_) => File::create(&ofile).expect("Could no creat .out file"),
    };

    // Creates temporary scratch file
    let iosch = tempfile().expect("Unable to creates temp file");

    let iothm = File::open("./thermo.lib").expect("Could not open thermo.lib");
    let iotrn = File::open("./trans.lib").expect("Could not open trans.lib");


    // Writes to .out file
    ioout.
    write_all( b"********************************\
    ***********************************************\n")
    .expect("Unable to write to file.");

    // Information
    ioout.write_all(b"         NASA-GLENN CHEMICAL EQUILIBRIUM PROGRAM CEA2, MAY 21, 2004
                   BY  BONNIE MCBRIDE AND SANFORD GORDON
     REFS: NASA RP-1311, PART I, 1994 AND NASA RP-1311, PART II, 1996\n")
    .expect("Unable to write to file");

    // Formatting
    ioout.
    write_all( b"********************************\
    ***********************************************\n")
    .expect("Unable to write to file.");

    let readok: bool = true;
    let newr: bool = false;
    let Nplt: i32 = 0;

    // Calls INPUT Function of CEA
    let mut caseok = true;
    let mut readok = true;
    INPUT(&mut readok, &mut caseok, &mut ensert)



}

fn INPUT(readok: &mut bool, caseok: &mut bool, ensert: &mut [String; 20]) {
    println!("{}", readok);
    println!("{}", caseok);
    println!("{:?}", ensert);
}


fn UTHERM() {
    // 

}