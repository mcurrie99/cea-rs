// pub mod config_constants;
pub mod files;

const CEA_FILE:&str = "FCEA2.exe";


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
