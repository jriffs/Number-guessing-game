#![allow(unused, unused_imports)]

use std::process;


pub mod convert_string;
pub mod generate_random;
pub mod check_guess;

#[derive(Debug)]
pub struct CliParams<'a> {
    pub rounds: i32,
    pub range: i32,
    somn: &'a str
}

impl<'a> CliParams<'a> {
    pub fn build(args: &'a [String]) -> Result<CliParams, &'static str> {
        if args.len() < 3 {
            return Err("please check the argments, should be 2 (search string & file name)");
        }
        let rounds = convert_string::convert_string_to_int(&args[1]).unwrap();
        let range = convert_string::convert_string_to_int(&args[2]).unwrap();
        Ok(CliParams {
            rounds,
            range,
            somn: &args[1]
        })
    }

    pub fn run<F>(&self, main_funtion: &'a F) where F: Fn() -> () {
        let rounds = self.rounds;
        for _ in 0..rounds {
            main_funtion();
        }
        process::exit(0);
    }
}

impl<'a> Iterator for CliParams<'a> {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.rounds > 0 {
            self.rounds -= 1;
            Some(self.rounds)
        } else {
            None
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    // CliParams can be built successfully with valid arguments
    #[test]
    fn test_cli_params_build_with_valid_arguments() {
        let args = vec![
            String::from("program"),
            String::from("10"),
            String::from("5")
        ];
        let cli_params = CliParams::build(&args).unwrap();
        assert_eq!(cli_params.rounds, 10);
        assert_eq!(cli_params.range, 5);
        assert_eq!(cli_params.somn, "10");
    }

    // CliParams cannot be built with less than 3 arguments
    #[test]
    fn test_cli_params_build_with_less_than_3_arguments() {
        let args = vec![
            String::from("program"),
            String::from("10")
        ];
        let cli_params = CliParams::build(&args);
        assert!(cli_params.is_err());
        assert_eq!(cli_params.unwrap_err(), "please check the argments, should be 2 (search string & file name)");
    }
}




