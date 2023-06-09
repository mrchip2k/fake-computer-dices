use std::process::exit;

use multiroll::Multiroll;

mod dice;
mod multiroll;
mod result_print;

fn main() {
    // TODO --help page

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
