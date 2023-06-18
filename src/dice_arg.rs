use crate::dice_types::DiceType;

pub struct AmountOutOfRangeError;


pub struct DiceArg {
    dice_type: DiceType,
    /// how many times this should be rolled
    amount: i32,
}


impl DiceArg {
    pub fn new(dice_type: &DiceType, amount: i32) -> Result<DiceArg, AmountOutOfRangeError> {
        if amount < 1 || amount > 100 {
            return Err(AmountOutOfRangeError);
        }

        return Ok(DiceArg {
            dice_type: dice_type.clone(),
            amount,
        });
    }

    pub fn get_dice_type(&self) -> DiceType {
        self.dice_type
    }

    pub fn get_amount(&self) -> i32 {
        self.amount
    }
}
