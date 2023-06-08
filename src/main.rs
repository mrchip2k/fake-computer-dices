use multiroll::Multiroll;

use crate::dice::Dice;

mod dice;
mod multiroll;
mod result_print;

fn main() {
    let mut rng = rand::thread_rng();

    // TODO delet
    // let dice = dice::Dice::D20;
    // println!("Rolling a {:?}: {}", format!("{:?}", dice), dice.roll(&mut rng));

    // TODO call new_from_cli_args() instead, when that function is ready
    let multiroll = Multiroll::new_test_default(Dice::D20);
    let result = multiroll.roll(&mut rng);
    result_print::simple::print_simple(multiroll.get_add(), result.0, &result.1);
}
