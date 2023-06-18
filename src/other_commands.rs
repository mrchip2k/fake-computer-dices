use std::collections::VecDeque;
use std::process::exit;

use strum::IntoEnumIterator;

use crate::dice_types::DiceType;

/// If args request the main program, this function does nothing.
/// Otherwise, the appropriate function is executed and the program is stopped.
pub fn try_run_other_commands(args: &VecDeque<String>) {
    if args.is_empty() {
        print_help();
        exit(1);
    }

    match &args[0][..] {
        "-h" | "--help" => {
            print_help();
            exit(0);
        }
        "-V" | "--version" => {
            print_version();
            exit(0);
        }
        _ => {}
    }
}


fn print_help() {
    println!("::: Help");
    println!("Your own set of virtual dices!");

    println!("Print this help page:");
    println!("\t{} --help | -h", env!("CARGO_PKG_NAME"));
    println!("Print version:");
    println!("\t{} --version | -V", env!("CARGO_PKG_NAME"));

    print!("Valid dices (case insensitive):");
    for dice in DiceType::iter() {
        print!(" {}", dice);
    }
    println!();

    println!("\n::: Examples");
    println!("Roll a single dice:");
    println!("\t{} d6", env!("CARGO_PKG_NAME"));
    println!("Roll multiple dice:");
    println!("\t{} d6 d12", env!("CARGO_PKG_NAME"));
    println!("Roll multiple dice and add a value (Negative is also allowed):");
    println!("\t{} d20 d20 +6", env!("CARGO_PKG_NAME"));
    println!("Multiple additional values are summed together:");
    println!("\t{} d4 d4 d4 d4 +15 -3", env!("CARGO_PKG_NAME"));
    println!("Multiple dice of the same type can be abbreviated:");
    println!("\t{} d4 x4", env!("CARGO_PKG_NAME"));

    println!("\nPrint with simplified and consistent formatting:");
    println!("\t{} --simple | -s d6 d4", env!("CARGO_PKG_NAME"));
}


fn print_version() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("repository: {}", env!("CARGO_PKG_REPOSITORY"));
}
