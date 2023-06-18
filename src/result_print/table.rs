use colored::{Color, ColoredString, Colorize};

use crate::dice_types::DiceType;
use crate::multiroll::MultirollResult;

pub fn print(data: &MultirollResult) {
    print_column_str("", "Type", "Rolled value", '╷');
    for i in data.rolls.iter().enumerate() {
        let dice = i.1;
        let i = i.0;
        print_dice(&dice.0, dice.1, i as i32 + 1);
    }
    print_column_str("", "", "", '╵');
    final_print(&data);
}


fn print_dice(dice: &DiceType, rolled_value: i32, dice_counter: i32) {
    print_column(
        dice_counter.to_string().color(Color::White),
        dice.to_string().color(dice.get_color()),
        rolled_value.to_string().bold(),
        '│');
}


fn print_column(
    counter_column: ColoredString,
    type_column: ColoredString,
    value_column: ColoredString,
    separator: char,
) {
    println!("{counter_column:>3}{separator}{type_column:<6}{separator}{value_column}");
}

fn print_column_str(
    counter_column: &str,
    type_column: &str,
    value_column: &str,
    separator: char,
) {
    print_column(counter_column.normal(),
                 type_column.normal(),
                 value_column.normal(),
                 separator);
}


fn final_print(data: &MultirollResult) {
    let message = match data.modifier {
        None => { data.simple_total.to_string() }
        Some(add_number) => {
            let simple_total = data.simple_total;
            let modified = simple_total + add_number;
            let add_word = if add_number > 0 { "add" } else { "subtract" };
            let add_number = add_number.abs();
            format!("rolled {simple_total}, {add_word} {add_number} = {modified}")
        }
    };

    println!("::: Final result: {}", message.bold());
}
