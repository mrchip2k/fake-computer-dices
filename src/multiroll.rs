use rand::rngs::ThreadRng;

use crate::dice::Dice;

pub struct Multiroll {
    add: i32,
    dices: Vec<Dice>,
}


impl Multiroll {
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


// ::: ::: ::: Parsing ::: ::: :::


enum Arg {
    Dice(Dice),
    Add(i32),
    DiceMultiplier(i32),
}


impl Multiroll {
    pub fn new_from_cli_args() -> Option<Multiroll> {
        let args = parse_individual_args();
        if args.is_none() {
            return None;
        }
        let args = args.unwrap();

        let validation = validate_sequence(&args);
        if validation.is_none() {
            return None;
        }
        let add = validation.unwrap();

        return Some(make_final(add, &args));
    }
}


fn parse_individual_args() -> Option<Vec<Arg>> {
    let args_str: Vec<String> = std::env::args().collect();
    let mut args_parsed: Vec<Arg> = Vec::new();

    for arg in &args_str[1..] {
        match arg.to_lowercase().as_str() {
            "d4" => { args_parsed.push(Arg::Dice(Dice::D4)) }
            "d6" => { args_parsed.push(Arg::Dice(Dice::D6)) }
            "d8" => { args_parsed.push(Arg::Dice(Dice::D8)) }
            "d10" => { args_parsed.push(Arg::Dice(Dice::D10)) }
            "d12" => { args_parsed.push(Arg::Dice(Dice::D12)) }
            "d20" => { args_parsed.push(Arg::Dice(Dice::D20)) }
            arg => {
                match arg.chars().nth(0).unwrap() {
                    'x' => {
                        match &arg[1..].parse::<i32>() {
                            Ok(value) => { args_parsed.push(Arg::DiceMultiplier(*value)) }
                            Err(_) => { return None; }
                        }
                    }
                    '+' | '-' => {
                        match arg.parse::<i32>() {
                            Ok(value) => { args_parsed.push(Arg::Add(value)) }
                            Err(_) => { return None; }
                        }
                    }
                    _ => { return None; }
                }
            }
        }
    }

    return Some(args_parsed);
}


fn validate_sequence(args: &Vec<Arg>) -> Option<i32> {
    let mut add = 0;

    let mut previous: Option<&Arg> = None;
    for arg in args {
        if !previous.is_none() { // first arg is never inherently wrong
            if !validate_one(&mut add, &arg, &previous.unwrap()) {
                return None;
            }
        }
        previous = Some(arg);
    }
    return Some(add);
}


fn validate_one(add: &mut i32, arg: &Arg, previous_arg: &Arg) -> bool {
    match arg {
        Arg::Add(value) => {
            *add += value;
            return true;
        }
        Arg::DiceMultiplier(value) => {
            if matches!(previous_arg, Arg::DiceMultiplier(_) | Arg::Add(_)) {
                println!("Error: Multiplier {} can only be written after a dice.", value);
                println!("Valid example: roll d6 x3");
                return false;
            } else if *value < 1 || *value > 100 {
                println!("Error: Dice count multiplier: {}", *value);
                println!("Must be positive, and can't be ridiculously large.");
                return false;
            } else if *value == 1 {
                println!("Warning: Useless multiplier \"x1\"");
            }
        }
        Arg::Dice(_) => {}
    }
    return true;
}


fn make_final(add: i32, args: &Vec<Arg>) -> Multiroll {
    let mut new = Multiroll { add, dices: Vec::new() };

    let mut previous_dice: Option<&Dice> = None;
    for arg in args {
        match arg {
            Arg::Add(_) => { /* already handled */ }
            Arg::Dice(value) => {
                new.dices.push(value.clone());
                previous_dice = Some(value);
            }
            Arg::DiceMultiplier(multiplier) => {
                for _ in 1..*multiplier {
                    new.dices.push(previous_dice.unwrap().clone());
                }
                previous_dice = None; // just in case, panic unwraps eventually, instead of doing questionable calculations
            }
        }
    }

    new
}