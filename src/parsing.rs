use std::collections::VecDeque;

use dice_arg::DiceArg;

use crate::dice_arg;
use crate::dice_types::DiceType;

pub enum ParseError {
    NoArgs,
    UnknownArgument { argument_index: i32 },
    DiceAmountOutOfRange { argument_index: i32 },
    IntParsing { argument_index: i32 },
}


#[readonly::make]
pub struct ParseResult {
    pub dice_args: Vec<DiceArg>,
    pub modifier: Option<i32>,
}


pub fn parse_args(args: &VecDeque<String>) -> Result<ParseResult, ParseError> {
    if args.is_empty() {
        return Err(ParseError::NoArgs);
    }
    let mut args = args.clone();
    let mut result = ParseResult { dice_args: Vec::new(), modifier: None };

    let mut argument_index = 0;
    while !args.is_empty() {
        match &args.pop_front().unwrap().to_lowercase()[..] {
            "d4" => { parse_dice(&mut result, &mut args, DiceType::D4, &mut argument_index)?; }
            "d6" => { parse_dice(&mut result, &mut args, DiceType::D6, &mut argument_index)?; }
            "d8" => { parse_dice(&mut result, &mut args, DiceType::D8, &mut argument_index)?; }
            "d10" => { parse_dice(&mut result, &mut args, DiceType::D10, &mut argument_index)?; }
            "d12" => { parse_dice(&mut result, &mut args, DiceType::D12, &mut argument_index)?; }
            "d20" => { parse_dice(&mut result, &mut args, DiceType::D20, &mut argument_index)?; }
            "coin" => { parse_dice(&mut result, &mut args, DiceType::Coin, &mut argument_index)?; }

            maybe_a_modifier => {
                match maybe_a_modifier.chars().nth(0) {
                    Some('+') | Some('-') => {
                        match &maybe_a_modifier.parse::<i32>() {
                            Ok(parsed_value) => {
                                if *parsed_value != 0 {
                                    match result.modifier {
                                        None => {
                                            result.modifier = Some(*parsed_value);
                                        }
                                        Some(previous_value) => {
                                            result.modifier = Some(previous_value + parsed_value);
                                        }
                                    }
                                }
                            }
                            Err(_) => {
                                return Err(ParseError::IntParsing { argument_index });
                            }
                        }
                    }
                    _ => { return Err(ParseError::UnknownArgument { argument_index }); }
                }
            }
        }

        argument_index += 1;
    }

    return Ok(result);
}


fn parse_dice(result: &mut ParseResult, args: &mut VecDeque<String>,
              dice_type: DiceType, argument_index: &mut i32)
              -> Result<(), ParseError> {
    let mut amount = 1;

    match args.pop_front() {
        None => {}
        Some(arg) => {
            match arg.chars().nth(0) {
                Some('x') => {
                    *argument_index += 1;
                    match &arg[1..].parse::<i32>() {
                        Ok(value) => {
                            amount = *value;
                        }
                        Err(_) => {
                            return Err(ParseError::IntParsing {
                                argument_index: *argument_index
                            });
                        }
                    }
                }

                _ => { args.push_front(arg); }
            }
        }
    }

    return match DiceArg::new(&dice_type, amount) {
        Ok(value) => {
            result.dice_args.push(value);
            Ok(())
        }
        Err(_) => {
            Err(ParseError::DiceAmountOutOfRange {
                argument_index: *argument_index
            })
        }
    };
}
