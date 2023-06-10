use colored::{Color, Colorize};

use crate::dice::Dice;

pub fn print_table(add: i32, total: i32, dices: &Vec<(i32, Dice)>) {
    println!("   ╷Type  ╷Rolled value");
    for i in dices.iter().enumerate() {
        print_dice(i.0 as i32 + 1, i.1.0, i.1.1);
    }
    println!("   ╵      ╵");
    final_print(total, add);
}

fn print_dice(dice_counter: i32, dice_value: i32, dice: Dice) {
    let msg = format!("{:>3}│{:<6}│{}",
                      dice_counter.to_string().color(Color::TrueColor { r: 128, g: 128, b: 128 }),
                      dice.to_string().color(dice.get_color()),
                      dice_value.to_string().bold());
    println!("{}", msg);
}

fn final_print(total: i32, add: i32) {
    let msg;
    if add != 0 {
        msg = format!("rolled {}, add {} = {}", total - add, add, total);
    } else {
        msg = format!("{}", total);
    }
    println!("::: Final result: {}", msg.bold());
}
