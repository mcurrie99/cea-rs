// Constant
const CEA_FILE:&str = "FCEA2.exe";


// Code for new interpretation
// pub mod config_constants;
pub mod files;

// Legacy Code
pub mod legacy;





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(result, 4);
    }
}
