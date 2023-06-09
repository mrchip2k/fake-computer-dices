use crate::dice::Dice;

pub fn print_simple(add: i32, total: i32, dices: &Vec<(i32, Dice)>) {
    for i in dices.iter().enumerate() {
        print_dice(i.0 as i32 + 1, i.1 .0, i.1 .1);
    }
    final_print(total, add);
}

fn print_dice(dice_counter: i32, dice_value: i32, dice: Dice) {
    println!("Dice n°{}\t{:?}\t{}", dice_counter, dice, dice_value);
}

fn final_print(total: i32, add: i32) {
    if add != 0 {
        println!("::: Final result: rolled {}, add {} = {}", total - add, add, total);
    } else {
        println!("::: Final result: {}", total);
    }
}
