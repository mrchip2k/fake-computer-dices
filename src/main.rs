use std::collections::VecDeque;
use std::process::exit;

use colored::Colorize;

use crate::parsing::ParseError;

mod dice_types;
mod result_print;
mod multiroll;
mod dice_arg;
mod parsing;
mod other_commands;


fn main() {
    let mut args: VecDeque<String> = std::env::args().collect();
    args.pop_front(); // don't care about program path

    other_commands::try_run_other_commands(&args);

    let use_simplified_print = matches!(&(args[0])[..], "--simple" | "-s");
    let argument_index_offset = if use_simplified_print {
        args.pop_front();
        2
    } else { 1 };

    let parsed_args = parsing::parse_args(&args);

    match parsed_args {
        Ok(args) => {
            let mut rng = rand::thread_rng();
            let result = multiroll::multiroll(&args, &mut rng);

            if use_simplified_print {
                result_print::simple::print(&result);
            } else {
                result_print::table::print(&result);
            }

            exit(0);
        }

        Err(error) => {
            print!("Error: ");
            match error {
                ParseError::NoArgs => {
                    println!("Command has no arguments.");
                    println!();
                    println!("Show help page:");
                    println!("\t{} --help | -h", env!("CARGO_PKG_NAME"));
                }
                ParseError::UnknownArgument { argument_index } => {
                    println!("Unknown argument.");
                    highlight_bad_argument(argument_index + argument_index_offset);
                    println!();
                    println!("Show help page:");
                    println!("\t{} --help | -h", env!("CARGO_PKG_NAME"));
                }
                ParseError::DiceAmountOutOfRange { argument_index } => {
                    println!("Dice count multiplier out of range.");
                    println!("Use a positive and non extreme value.");
                    highlight_bad_argument(argument_index + argument_index_offset);
                }
                ParseError::IntParsing { argument_index } => {
                    println!("Could not parse parameter as integer.");
                    highlight_bad_argument(argument_index + argument_index_offset);
                }
            }
        }
    }

    exit(1);
}


fn highlight_bad_argument(bad_index: i32) {
    println!("\nBad argument here:");
    let args: Vec<String> = std::env::args().collect();
    for arg in args.iter().enumerate() {
        let msg = if arg.0 as i32 == bad_index {
            arg.1.on_bright_red().black().bold().underline()
        } else {
            arg.1.normal()
        };

        print!("{msg} ");
    }
    println!();
}