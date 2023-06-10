use std::fmt;
use std::fmt::Formatter;

use rand::Rng;
use rand::rngs::ThreadRng;

#[derive(Debug, Copy, Clone, strum::EnumIter)]
pub enum Dice {
    Coin,
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
}

impl Dice {
    pub fn roll(self, rng: &mut ThreadRng) -> i32 {
        match self {
            Dice::Coin => { rng.gen_range(0..=1) }
            _ => { rng.gen_range(1..=self.get_dice_max()) }
        }
    }

    // What is the highest number this type of dice can roll?
    fn get_dice_max(self) -> i32 {
        match self {
            Dice::D4 => 4,
            Dice::D6 => 6,
            Dice::D8 => 8,
            Dice::D10 => 10,
            Dice::D12 => 12,
            Dice::D20 => 20,
            Dice::Coin => 1, // not actually used
        }
    }
}

impl fmt::Display for Dice {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
