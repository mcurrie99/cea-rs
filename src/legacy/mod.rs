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
const MAXNC: usize = 300;
const NCOL: usize = 8;
const MAXMAT: usize = 50;
const MAXTR: usize = 40;
const MAXR: usize = 24;
const MAXEL: usize = 20;
const MAXNG: usize = 500;
const MAXMIX: usize = 52;
const MAXT: usize = 51;
const MAXPV: usize = 26;

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
    // Computational Block of Variables (cea.inc)
    Enn: Option<f64>,
    Ennl: Option<f64>,
    Enlsav: Option<f64>,
    Ensave: Option<f64>,
    Sumn: Option<f64>,
    Deln: Option<[f64; MAXNGC]>,
    En: Option<[[f64; MAXNGC]; NCOL]>,

    // Index Block of Variables (cea.inc)
    Ip: Option<i64>,
    Iplt: Option<i64>,
    It: Option<i64>,
    Nc: Option<i64>,
    Ng: Option<i64>,
    Ngp1: Option<i64>,
    Nlm: Option<i64>,
    Nplt: Option<i64>,
    Nof: Option<i64>,
    Nomit: Option<i64>,
    Nonly: Option<i64>,
    Np: Option<i64>,
    Npr: Option<i64>,
    Npt: Option<i64>,
    Ngc: Option<i64>,
    Nsert: Option<i64>,
    Nspr: Option<i64>,
    Nspx: Option<i64>,
    Nt: Option<i64>,
    Jcond: Option<[i64; 45]>,
    Jx: Option<[i64; MAXEL]>,
    Nfla: Option<[i64; MAXR]>,
    Ifz: Option<[i64; MAXNC]>,

    // Input Block of Variables (cea.inc)
    Cpmix: Option<f64>,
    Wmix: Option<f64>,
    Bcheck: Option<f64>,
    Am: Option<[f64; 2]>,
    Hpp: Option<[f64; 2]>,
    Vmin: Option<[f64; 2]>,
    Vpls: Option<[f64; 2]>,
    Wp: Option<[f64; 2]>,
    Atmwt: Option<[f64; 100]>,
    Oxf: Option<[f64; MAXMIX]>,
    P: Option<[f64; MAXPV]>,
    Rh: Option<[f64; 2]>,
    T: Option<[f64; MAXT]>,
    V: Option<[f64; MAXPV]>,
    Valnce: Option<[f64; 100]>,
    B0p: Option<[[f64; MAXEL]; 2]>,

    // Miscellaneous Integers Block of Variables (cea.inc)
    Imat: Option<i64>,
    Iq1: Option<i64>,
    Isv: Option<i64>,
    Jliq: Option<i64>,
    Jsol: Option<i64>,
    Lsave: Option<i64>,
    Msing: Option<i64>,

    // Miscellaneous Boolean Block of Variables (cea.inc)
    Convg: Option<bool>,
    Debug: Option<[bool; NCOL]>,
    Detdbg: Option<bool>,
    Detn: Option<bool>,
    Eql: Option<bool>,
    Gonly: Option<bool>,
    Hp: Option<bool>,
    Ions: Option<bool>,
    Massf: Option<bool>,
    Moles: Option<bool>,
    Newr: Option<bool>,
    Pderiv: Option<bool>,
    Shock: Option<bool>,
    Short: Option<bool>,
    Siunit: Option<bool>,
    Sp: Option<bool>,
    Tp: Option<bool>,
    Trnspt: Option<bool>,
    Vol: Option<bool>,

    // Miscellaneous Floating-Point Block of Variables (cea.inc)
    Avgdr: Option<f64>,
    Boltz: Option<f64>,
    Eqrat: Option<f64>,
    Hsub0: Option<f64>,
    Oxfl: Option<f64>,
    Pi: Option<f64>,
    Pp: Option<f64>,
    R: Option<f64>,
    Rr: Option<f64>,
    Size: Option<f64>,
    S0: Option<f64>,
    Tln: Option<f64>,
    Tm: Option<f64>,
    Trace: Option<f64>,
    Tt: Option<f64>,
    Viscns: Option<f64>,
    Vv: Option<f64>,
    Atwt: Option<[f64; MAXEL]>,
    B0: Option<[f64; MAXEL]>,
    X: Option<[f64; MAXMAT]>,
    A: Option<[[f64; MAXEL]; MAXNGC]>,
    G: Option<[[f64; MAXMAT]; MAXMAT+1]>,

    // Strings (Character Arrays) Block of Variabels (cea.inc)
    Elmt: Option<[String; MAXEL]>,
    Ratom: Option<[[String; MAXR]; 12]>,
    Symbol: Option<[String; 100]>,
    Fmt: Option<[String; 30]>,
    Fox: Option<[String; MAXR]>,
    Thdate: Option<String>,
    Case: Option<String>,
    Energy: Option<[String; MAXR]>,
    Omit: Option<[String; MAXNGC+1]>, // TODO: Check this
    Pltvar: Option<[String; 20]>,
    Prod: Option<[String; MAXNGC+1]>, // TODO: Check this
    Rname: Option<[String; MAXR]>,
    Pfile: Option<String>,

    // PRTOUT Block of Variables (cea.inc)
    Cpr: Option<[f64; NCOL]>,
    Dlvpt: Option<[f64; NCOL]>,
    Dlvtp: Option<[f64; NCOL]>,
    Gammas: Option<[f64; NCOL]>,
    Hsum: Option<[f64; NCOL]>,
    Ppp: Option<[f64; NCOL]>,
    Ssum: Option<[f64; NCOL]>,
    Totn: Option<[f64; NCOL]>,
    Ttt: Option<[f64; NCOL]>,
    Vlm: Option<[f64; NCOL]>,
    Wm: Option<[f64; NCOL]>,
    Pltout: Option<[[f64;500];20]>,

    // Reaction REACTN Block of Variables (cea.inc)
    Nreac: Option<i64>,
    Jray: Option<[i64; MAXR]>,
    Dens: Option<[f64; MAXR]>,
    Enth: Option<[f64; MAXR]>,
    Pecwt: Option<[f64; MAXR]>,
    Rmw: Option<[f64; MAXR]>,
    Rtemp: Option<[f64; MAXR]>,
    Rnum: Option<[[f64; MAXR]; 12]>,

    // Thermal THERM Block of Variables (cea.inc)
    Cpsum: Option<f64>,
    Cft: Option<[[f64; MAXNC]; 9]>,
    Coef: Option<[[[f64; MAXNG]; 9]; 3]>,
    Temp: Option<[[f64; 2]; MAXNC]>,
    Cp: Option<[f64; MAXNGC]>,
    H0: Option<[f64; MAXNGC]>,
    Mu: Option<[f64; MAXNGC]>,
    Mw: Option<[f64; MAXNGC]>,
    S: Option<[f64; MAXNGC]>,
    Tg: Option<[f64; 4]>,

    // Rocket ROCKT Block of Variables (cea.inc)
    Iopt: Option<i64>,
    Isup: Option<i64>,
    Nfz: Option<i64>,
    Npp: Option<i64>,
    Nsub: Option<i64>,
    Nsup: Option<i64>,
    Area: Option<bool>,
    Debugf: Option<bool>,
    Fac: Option<bool>,
    Froz: Option<bool>,
    Page1: Option<bool>,
    Rkt: Option<bool>,
    Acat: Option<f64>,
    Awt: Option<f64>,
    Cstr: Option<f64>,
    Tcest: Option<f64>,
    Ma: Option<f64>,
    Aeat: Option<[f64; NCOL]>,
    App: Option<[f64; NCOL]>,
    Pcp: Option<[f64; 2*NCOL]>,
    Sonvel: Option<[f64; NCOL]>,
    Spim: Option<[f64; NCOL]>,
    Subar: Option<[f64; 13]>,
    Supar: Option<[f64; 13]>,
    Vmoc: Option<[f64; NCOL]>,

    // Shockwave values SHOCKS Block of Variables (cea.inc)
    Nsk: Option<i64>,
    Incdeq: Option<bool>,
    Incdfz: Option<bool>,
    Refleq: Option<bool>,
    Reflfz: Option<bool>,
    Shkdbg: Option<bool>,
    U1: Option<[f64; NCOL]>,
    Mach1: Option<[f64; NCOL]>,
    A1: Option<f64>,
    Gamma1: Option<f64>,

    // Transport Properties TRNP Block of Variables (cea.inc)
    Nm: Option<i64>,
    Nr: Option<i64>,
    Ntape: Option<i64>,
    Ind: Option<[i64; MAXTR]>,
    Jcm: Option<[i64; MAXEL]>,
    Cprr: Option<[f64; MAXTR]>,
    Con: Option<[f64; MAXTR]>,
    Wmol: Option<[f64; MAXTR]>,
    Xs: Option<[f64; MAXTR]>,
    Eta: Option<[[f64; MAXTR]; MAXTR]>,
    Stc: Option<[[f64; MAXTR]; MAXTR]>,

    // Transport Properties TRPTS Block of Variables (cea.inc)
    Coneql: Option<[f64; NCOL]>,
    Confro: Option<[f64; NCOL]>,
    Cpeql: Option<[f64; NCOL]>,
    Cpfro: Option<[f64; NCOL]>,
    Preql: Option<[f64; NCOL]>,
    Prfro: Option<[f64; NCOL]>,
    Vis: Option<[f64; NCOL]>,


    // TODO: Figure out what they all do
    // Debug: Option<[bool; NCOL]>,
    // Nonly: Option<isize>,
    // Nomit: Option<isize>,
    // Nsert: Option<isize>,
    // Trace: Option<isize>,
    // Short: Option<bool>,
    // Massf: Option<bool>,
    // Nplt: Option<isize>,
    // Siunit: Option<bool>,
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
            // Computational Block of Variables (cea.inc)
            Enn: None,
            Ennl: None,
            Enlsav: None,
            Ensave: None,
            Sumn: None,
            Deln: None,
            En: None,

            // Index Block of Variables (cea.inc)
            Ip: None,
            Iplt: None,
            It: None,
            Jcond: None,
            Jx: None,
            Nc: None,
            Ng: None,
            Ngp1: None,
            Nlm: None,
            Nplt: None,
            Nof: None,
            Nomit: None,
            Nonly: None,
            Np: None,
            Npr: None,
            Npt: None,
            Ngc: None,
            Nsert: None,
            Nspr: None,
            Nspx: None,
            Nt: None,
            Nfla: None,
            Ifz: None,

            // Input Block of Variables (cea.inc)
            Am: None,
            B0p: None,
            Cpmix: None,
            Hpp: None,
            Vmin: None,
            Vpls: None,
            Wmix: None,
            Wp: None,
            Atmwt: None,
            Bcheck: None,
            Oxf: None,
            P: None,
            Rh: None,
            T: None,
            V: None,
            Valnce: None,

            // Miscellaneous Integers Block of Variables (cea.inc)
            Imat: None,
            Iq1: None,
            Isv: None,
            Jliq: None,
            Jsol: None,
            Lsave: None,
            Msing: None,

            // Miscellaneous Booleans Block of Variables (cea.inc)
            Convg: None,
            Debug: None,
            Detdbg: None,
            Detn: None,
            Eql: None,
            Gonly: None,
            Hp: None,
            Ions: None,
            Massf: None,
            Moles: None,
            Newr: None,
            Pderiv: None,
            Shock: None,
            Short: None,
            Siunit:None,
            Sp: None,
            Tp: None,
            Trnspt: None,
            Vol: None,

            // Miscellaneous Floating Points Block of Variables (cea.inc)
            A: None,
            Atwt: None,
            Avgdr: None,
            Boltz: None,
            B0: None,
            Eqrat: None,
            G: None,
            Hsub0: None,
            Oxfl: None,
            Pi: None,
            Pp: None,
            R: None,
            Rr: None,
            Size: None,
            S0: None,
            Tln: None,
            Tm: None,
            Trace: None,
            Tt: None,
            Viscns: None,
            Vv: None,
            X: None,

            
            // Strings (Character Arrays) Block of Variables (cea.inc)
            Case: None,
            Elmt: None,
            Energy: None,
            Fmt: None,
            Fox: None,
            Omit: None,
            Pltvar: None,
            Prod: None,
            Ratom: None,
            Rname: None,
            Symbol: None,
            Thdate: None,
            Pfile: None,

            // PRTOUT Block of Variables (cea.inc)
            Cpr: None,
            Dlvpt: None,
            Dlvtp: None,
            Gammas: None,
            Hsum: None,
            Ppp: None,
            Ssum: None,
            Totn: None,
            Ttt: None,
            Vlm: None,
            Wm: None,
            Pltout: None,

            // Reaction REACTN Block of Variables (cea.inc)
            Dens: None,
            Enth: None,
            Pecwt: None,
            Rmw: None,
            Rnum: None,
            Rtemp: None,
            Jray: None,
            Nreac: None,

            // Thermal THERM Block of Variables (cea.inc)
            Cft: None,
            Coef: None,
            Cp: None,
            Cpsum: None,
            H0: None,
            Mu: None,
            Mw: None,
            S: None,
            Temp: None,
            Tg: None,

            // Rocket ROCKT Block of Variables (cea.inc)
            Acat: None,
            Aeat: None,
            App: None,
            Awt: None,
            Cstr: None,
            Ma: None,
            Pcp: None,
            Sonvel: None,
            Spim: None,
            Subar: None,
            Supar: None,
            Vmoc: None,
            Tcest: None,
            Area: None,
            Iopt: None,
            Debugf: None,
            Fac: None,
            Froz: None,
            Isup: None,
            Nfz: None,
            Npp: None,
            Nsub: None,
            Nsup: None,
            Page1: None,
            Rkt: None,

            // Shockwave values SHOCKS Block of Variables (cea.inc)
            U1: None,
            Mach1: None,
            A1: None,
            Gamma1: None,
            Incdeq: None,
            Incdfz: None,
            Refleq: None,
            Reflfz: None,
            Shkdbg: None,
            Nsk: None,

            // Transport Properties TRNP Block of Variables (cea.inc)
            Cprr: None,
            Con: None,
            Eta: None,
            Wmol: None,
            Xs: None,
            Stc: None,
            Ind: None,
            Jcm: None,
            Nm: None,
            Nr: None,
            Ntape: None,

            // Transport Properties TRPTS Block of Variables (cea.inc)
            Coneql: None,
            Confro: None,
            Cpeql: None,
            Cpfro: None,
            Preql: None,
            Prfro: None,
            Vis: None,
            
            // TODO: Say what these actually are in documentation
            // Debug: None,
            // Nonly: None,
            // Nomit: None,
            // Nsert: None,
            // Trace: None,
            // Short: None,
            // Massf: None,
            // Nplt: None,
            // Siunit: None,
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
    let mut input_vars = INPUT_vals::initialize();
    INPUT(&mut readok, &mut caseok, &mut ensert, &mut data, &mut input_vars);



}

struct INPUT_vals {
    // Locally saved variables for the "Subroutine"
    cin: Option<[String; MAXNGC]>,
    code: Option<String>,
    cx1: Option<String>,
    cx15: Option<String>,
    cx2: Option<String>,
    cx3: Option<String>,
    cx4: Option<String>,
    denmtr: Option<f64>,
    dpin: Option<f64>,
    eqrats: Option<bool>,
    eratio: Option<f64>,
    hr: Option<f64>,
    i: Option<i64>,
    ifrmla: Option<i64>,
    ii: Option<i64>,
    in_cea: Option<i64>,
    incd: Option<bool>,
    iv: Option<i64>,
    ix: Option<i64>,
    j: Option<i64>,
    jj: Option<i64>,
    k: Option<i64>,
    lcin: Option<[i64; MAXNGC]>,
    mix: Option<f64>,
    ncin: Option<i64>,
    nmix: Option<i64>,
    phi: Option<bool>,
    pltdat: Option<bool>,
    reacts: Option<bool>,
    refl: Option<bool>,
    ur: Option<f64>,
    xyz: Option<f64>
}

impl INPUT_vals {
    fn initialize() -> Self {
        let data = Self {
            cin: Some(std::array::from_fn(|_| String::new())),
            code: Some(String::new()),
            cx1: Some(String::new()),
            cx15: Some(String::new()),
            cx2: Some(String::new()),
            cx3: Some(String::new()),
            cx4: Some(String::new()),
            denmtr: None,
            dpin: None,
            eqrats: None,
            eratio: None,
            hr: None,
            i: None,
            ifrmla: None,
            ii: None,
            in_cea: None,
            incd: None,
            iv: None,
            ix: None,
            j: None,
            jj: None,
            k: None,
            lcin: None,
            mix: None,
            ncin: None,
            nmix: None,
            phi: None,
            pltdat: None,
            reacts: None,
            refl: None,
            ur: None,
            xyz: None
        };

        return data;
    }
}

fn INPUT(readok: &mut bool, caseok: &mut bool, ensert: &mut [String; 20], data: &mut cea_data, input_vars: &mut INPUT_vals) {
  
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
    data.Trace = Some(0.0);
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