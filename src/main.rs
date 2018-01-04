extern crate clap;
extern crate rand;

use clap::{Arg, App};

use rand::Rng;

#[derive(Debug, PartialEq, Eq)]
struct Dice {
    number: u32,
    sides: u32,
}

impl Dice {
    fn parse(dice: &str) -> Option<Dice> {
        let tokens: Vec<&str> = dice.split("d").collect();

        if tokens.len() != 2 {
            return None;
        }

        let number = tokens[0].parse().unwrap();
        let sides = tokens[1].parse().unwrap();

        Some(Dice { number, sides })
    }
}


fn main() {
    let matches = App::new("rolldice")
        .arg(Arg::with_name("dice")
             .required(true))
        .get_matches();

    let dice = matches.value_of("dice").unwrap();
    let dice = Dice::parse(dice);

    println!("{:?}", dice);
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dice_parse_none() {
        assert_eq!(Dice::parse(""), None);
        assert_eq!(Dice::parse("something"), None);
    }

    #[test]
    fn dice_parse_correct() {
        assert_eq!(
            Dice::parse("6d100"),
            Some(Dice { number: 6, sides: 100 }));
        assert_eq!(
            Dice::parse("1d5"),
            Some(Dice { number: 1, sides: 5}));
    }
}
