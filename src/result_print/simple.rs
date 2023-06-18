use crate::multiroll::MultirollResult;

pub fn print(data: &MultirollResult) {
    for dice in &data.rolls {
        println!("{:8}{}", dice.0.to_string(), dice.1)
    }
}
