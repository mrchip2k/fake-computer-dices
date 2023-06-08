use std::env;

use rand::rngs::ThreadRng;

use crate::dice::Dice;

pub struct Multiroll {
    add: i32,
    dices: Vec<Dice>,
}

impl Multiroll {
    pub fn new_default() -> Multiroll {
        Multiroll {
            add: 0,
            dices: Vec::new(),
        }
    }

    pub fn new_from_cli_args() -> Multiroll {
        let args: Vec<String> = env::args().collect();

        let mut new = Multiroll::new_default();

        todo!();

        new
    }

    pub fn new_test_default(dice: Dice) -> Multiroll {
        Multiroll {
            add: 0,
            dices: vec![
                dice,
                Dice::D4,
                Dice::D6,
                Dice::D8,
                Dice::D10,
                Dice::D12,
                Dice::D20,
            ],
        }
    }

    pub fn roll(&self, rng: &mut ThreadRng) -> (i32, Vec<(i32, Dice)>) {
        let mut sum = self.add;
        let mut rolled_dices = Vec::new();

        for dice in &self.dices {
            let rolled_value = dice.roll(rng);
            sum += rolled_value;
            rolled_dices.push((rolled_value, dice.clone()));
        }

        (sum, rolled_dices)
    }

    pub fn get_add(self) -> i32 {
        self.add
    }
}
