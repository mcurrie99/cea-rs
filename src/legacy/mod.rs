// NOTES FOR MATT DURING DEVELOPMENT
// - Fortran does not seem to have dynamically sizing arrays
//   Ensure to use caution since we have no check, we can implement a 
//   structure to do this but it would be unecessary.
// - PLEASE READ THE NOTES
// - Keep the structure for variables that will be passed between the functions
// - Create a structure for each function that allows for the local variables to remain
// - We can also create static variables and use unsafe rust but we should try
//   to stray from this.


// Support for legacy CEA will be added here so that the application
// Can be ran in a way that is compatible with old versions
use std::io::{self, Write, Read};
use std::fs::{File, OpenOptions};
use tempfile::tempfile;


// The following paramters set the maximum dimensions for many variables
//  They are definedi n part 2, pages 39 of the manuals, NASA RP-1311
//  The variable NCOL set the number of columns in the output. It may
//  be increased for wider paper or smaller fonts
const MAXNGC: usize = 600;
const MAXNC: i32 = 300;
const NCOL: usize = 8;
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

// This structure is going to be how we simulate global variables that need to
// be mutable by just continuously passing around the structure
// PSA: I know this is probably a terrible implementation for doing this
// but it is going to work and it's the legacy version so who cares
struct cea_data {
    // Variables
    // TODO: Figure out what they all do
    Debug: Option<[bool; NCOL]>,
    Nonly: Option<isize>,
    Nomit: Option<isize>,
    Nsert: Option<isize>,
    Trace: Option<isize>,
    Short: Option<bool>,
    Massf: Option<bool>,
    Nplt: Option<isize>,
    Siunit: Option<bool>,
    pltdat: Option<bool>,

    // .inp, .out, .plt files
    ioout: Option<File>,
    ioinp: Option<File>,
    ioplt: Option<File>,

    // Library and Scrath Files
    iothm: Option<File>,
    iotrn: Option<File>,
    iosch: Option<File>,

}

impl cea_data {
    fn initialize() -> Self {
        let data = cea_data {
            
            // TODO: Say what these actually are in documentation
            Debug: None,
            Nonly: None,
            Nomit: None,
            Nsert: None,
            Trace: None,
            Short: None,
            Massf: None,
            Nplt: None,
            Siunit: None,
            pltdat: None,
            
            // Files that will be handled
            ioout: None,
            ioinp: None,
            ioplt: None,

            // Library and Scratch Files
            iothm: None,
            iotrn: None,
            iosch: None,
        };

        return data;
    }
}


// STATIC VARIABLES ADDED
// static mut NONLY: i32 = 0;


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

    let mut data: cea_data = cea_data::initialize();

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
    data.ioinp = Some(File::open(&infile).expect(err_msg.as_str()));

    // Opens or creates the outfile
    data.ioout = match OpenOptions::new().read(true).write(true).open(&ofile) {
        Ok(file) => Some(file),
        Err(_) => Some(File::create(&ofile).expect("Could no creat .out file")),
    };

    // Creates temporary scratch file
    data.iosch = Some(tempfile().expect("Unable to creates temp file"));
    data.iothm = Some(File::open("./thermo.lib").expect("Could not open thermo.lib"));
    data.iotrn = Some(File::open("./trans.lib").expect("Could not open trans.lib"));


    // Writes to .out file
    data.ioout
        .as_mut()
        .expect("Output File was not initialized correctly")
        .write_all( b"********************************\
        ***********************************************\n")
        .expect("Unable to write to file.");

    // Information
    data.ioout
    .as_mut()
    .expect("Ouput file was not intialized correctly")
    .write_all(b"         NASA-GLENN CHEMICAL EQUILIBRIUM PROGRAM CEA2, MAY 21, 2004
                   BY  BONNIE MCBRIDE AND SANFORD GORDON
     REFS: NASA RP-1311, PART I, 1994 AND NASA RP-1311, PART II, 1996\n")
    .expect("Unable to write to file");

    // Formatting for file
    data.ioout
    .as_mut()
    .unwrap()
    .write_all( b"********************************\
    ***********************************************\n")
    .expect("Unable to write to file.");

    let readok: bool = true;
    let newr: bool = false;
    let Nplt: i32 = 0;

    // Calls INPUT Function of CEA
    let mut caseok = true;
    let mut readok = true;

    // label_100(&mut readok, &mut caseok, &mut ensert);

    INPUT(&mut readok, &mut caseok, &mut ensert, &mut data)



}

struct INPUT_vals {
    
}

fn INPUT(readok: &mut bool, caseok: &mut bool, ensert: &mut [String; 20], data: &mut cea_data) {
  
    println!("{}", readok);
    println!("{}", caseok);
    println!("{:?}", ensert);

    // Local Variables Strings
    let mut cin: [String; MAXNGC]  = std::array::from_fn(|_| String::new());
    let mut ensert: [String; 20] = Default::default();
    let mut code: String = String::new();
    let mut cx4 = String::new();
    let mut cx1 = String::new();
    let mut cx2 = String::new();
    let mut cx3 = String::new();
    let mut lc = String::new();
    let mut uc = String::new();

    // Local Variables Bools
    let eqrats: bool;
    let incd: bool;
    let phi: bool;
    let pltdat: bool;
    let reacts: bool;
    let refl: bool;

    // Integers
    let i: isize;
    let ifrmla: isize;
    let ii: isize;
    let in_cea: isize; // _cea to avoid the in keyword
    let iv:  isize;
    let ix: isize;
    let j: isize;
    let jj: isize;
    let k: isize;
    let lcin: [isize; MAXNGC];
    let ncin: isize;
    let nmix: isize;
    let INDEX: isize;
    
    // Floating Points
    let denmtr: f64;
    let dpin: [f64; MAXNGC];
    let eratio: f64;
    let hr: f64;
    let mix: [f64; MAXNGC];
    let ur: f64;
    let xyz: f64;
    let DABS: f64;
    let DMIN1: f64;
    let DSQRT: f64;

    // Execution of Code
    data.ioout
        .as_mut()
        .expect("1 Output File was not intialized properly")
        .write(b"\n\n")
        .expect("Could not write to output file");

    *caseok = true;
    data.Nonly = Some(0);
    data.Nomit = Some(0);
    data.Nsert = Some(0);
    reacts = false;
    data.Trace = Some(0);
    data.Short = Some(false);
    data.Massf = Some(false);

    // Sets debug data
    data.Debug = Some([false; NCOL]);

    data.Nplt = Some(0);
    data.Siunit = Some(true);
    data.pltdat = Some(false);

    fn label_100(data: &mut cea_data) {
        // TODO: Implement INFREE Function
        INFREE();

        // code = cin[0]

    }

    label_100(data);

}

struct UTHERM_vals {

}

// TODO: Implement Function
fn UTHERM() {
    // 

}

struct INFREE_vals {

}

// TODO: Implement Function
fn INFREE() {

}

// label Functions
// fn label_100(readok: &mut bool, caseok: &mut bool, ensert: &mut [String; 20]) {
//     let Iplt = 0;
//     let Nplt = 0;

//     INPUT(readok, caseok, ensert);

// }