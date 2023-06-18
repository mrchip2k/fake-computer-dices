use rand::rngs::ThreadRng;

use crate::dice_types::DiceType;
use crate::parsing::ParseResult;

#[readonly::make]
pub struct MultirollResult {
    /// field .1 is the random value that came out
    pub rolls: Vec<(DiceType, i32)>,
    /// all rolls (rolls.1) simply added together
    pub simple_total: i32,
    /// the number the user requested to add
    pub modifier: Option<i32>,
}


pub fn multiroll(input: &ParseResult, rng: &mut ThreadRng) -> MultirollResult {
    let mut rolls = Vec::new();
    let mut summed_rolls = 0;
    let modifier = match input.modifier {
        None => { None }
        Some(value) => {
            if value == 0 { Some(0) } else { Some(value) }
        }
    };

    for dice_arg in &input.dice_args {
        for _ in 0..dice_arg.get_amount() {
            let dice = dice_arg.get_dice_type().clone();
            let roll = dice.roll(rng);
            summed_rolls += roll;
            rolls.push((dice, roll));
        }
    }

    MultirollResult { rolls, simple_total: summed_rolls, modifier }
}