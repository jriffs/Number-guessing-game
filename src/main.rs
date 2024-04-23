#![allow(unused, unused_imports)]

use std::{env, process};
use guessing_game::{CliParams, 
    convert_string::convert_string_to_int, 
    check_guess::check_user_input, generate_random::generate_random_int};
use std::io;
use rand::{Rng, thread_rng};


fn main() {
    let args: Vec<String> = env::args().collect(); 
    let config_var = CliParams::build(&args);
    let mut config_var = match config_var {
        Ok(p) => p,
        Err(msg) => {
            eprintln!("{}", msg);
            process::exit(1);
        }
    };
    let read_line = || {
        println!("Please enter your guess ...");
        // println!("we are just testing something");
        let mut input = String::new();
        let random = thread_rng().gen_range(1..=config_var.range);
        match io::stdin().read_line(&mut input) {
            Ok(p) => {
                if input.contains("end") {
                    println!("Cli tool is shutting down on command");
                    process::exit(0);
                }
                let users_guess = convert_string_to_int(input.trim()).unwrap();
                if check_user_input(users_guess, random) == true {
                    println!("correct 👏👏👏");
                } else {
                    println!("wrong 😭");
                }
            },
            Err(e) => println!("There was an error: {e}")
        }
    };
    config_var.run(&read_line);
}
