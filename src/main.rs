use std::process::exit;

use strum::IntoEnumIterator;

use multiroll::Multiroll;

use crate::dice::Dice;

mod dice;
mod multiroll;
mod result_print;


fn main() {
    try_run_different_modes();

    let mut rng = rand::thread_rng();

    let multiroll = Multiroll::new_from_cli_args();
    if multiroll.is_none() {
        println!("Error: Could not parse arguments.");
        exit(1);
    }
    let multiroll = multiroll.unwrap();

    let result = multiroll.roll(&mut rng);

    result_print::simple::print_simple(multiroll.get_add(), result.0, &result.1);
}


fn try_run_different_modes() {
    let argv: Vec<String> = std::env::args().collect();

    if argv.len() <= 1 {
        print_help();
        exit(1);
    }

    if argv.len() == 2 {
        match &argv[1][..] {
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
}


fn print_help() {
    println!("::: Help");
    println!("Your own set of virtual dices!");

    println!("Print this help page:");
    println!("\t{} --help | -h", env!("CARGO_PKG_NAME"));
    println!("Print version:");
    println!("\t{} --version | -V", env!("CARGO_PKG_NAME"));

    print!("Valid dices (case insensitive):");
    for dice in Dice::iter() {
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
}


fn print_version() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("repository: {}", env!("CARGO_PKG_REPOSITORY"));
}