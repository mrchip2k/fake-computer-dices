use std::fmt;
use std::fmt::Formatter;

use colored::Color;
use rand::Rng;
use rand::rngs::ThreadRng;

#[derive(Debug, Copy, Clone, strum::EnumIter)]
pub enum DiceType {
    Coin,
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
}


impl DiceType {
    pub fn roll(self, rng: &mut ThreadRng) -> i32 {
        match self {
            DiceType::Coin => { rng.gen_range(0..=1) }
            _ => { rng.gen_range(1..=self.get_dice_max()) }
        }
    }


    pub fn get_color(self) -> Color {
        match self {
            DiceType::Coin => { Color::BrightYellow }
            DiceType::D4 => { Color::Red }
            DiceType::D6 => { Color::Yellow }
            DiceType::D8 => { Color::BrightYellow }
            DiceType::D10 => { Color::Green }
            DiceType::D12 => { Color::Cyan }
            DiceType::D20 => { Color::Magenta }
        }
    }


    // What is the highest number this type of dice can roll?
    fn get_dice_max(self) -> i32 {
        match self {
            DiceType::D4 => 4,
            DiceType::D6 => 6,
            DiceType::D8 => 8,
            DiceType::D10 => 10,
            DiceType::D12 => 12,
            DiceType::D20 => 20,
            DiceType::Coin => 1, // not actually used
        }
    }
}


impl fmt::Display for DiceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
