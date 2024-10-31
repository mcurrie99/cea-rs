pub mod thermo {
    // Imports
    use std::fs::File;

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
    pub fn load_data(filepath: &str) -> File {
        let file = File::open(filepath).expect("Could not open file");
        file
    }

}