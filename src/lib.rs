extern crate rand;

use rand:: Rng;
use rand::distributions::{Range, IndependentSample};


#[derive(Debug, PartialEq, Eq)]
pub struct Dice {
    pub number: u32,
    pub sides: u32,
}

impl Dice {
    /// Create a Dice instance given a standard dice format.
    /// Expected format is <number>d<sides> where <number>
    /// is the number of dice to roll and <sides> is the number
    /// of sides per dice rolled.
    ///
    /// ```
    /// let result = rolldice::Dice::parse("4d6").unwrap();
    ///
    /// assert_eq!(result, rolldice::Dice { number: 4, sides: 6 });
    /// ```
    pub fn parse(dice: &str) -> Result<Dice, String> {
        let tokens: Vec<&str> = dice.split("d").collect();

        if tokens.len() != 2 {
            return Err(format!("Invalid dice format '{}'", dice));
        }

        let number = match tokens[0].parse() {
            Ok(num) => num,
            Err(_) => return Err(format!("Invalid number of dice '{}'", tokens[0])),
        };
        let sides = match tokens[1].parse() {
            Ok(num) => num,
            Err(_) => return Err(format!("Invalid number of sides '{}'", tokens[1])),
        };

        Ok(Dice { number, sides })
    }

    /// Generate a dice role from a Dice instance. Requires a random
    /// number generator from the "rand" crate to be passed in.
    ///
    /// ```
    /// extern crate rand;
    /// extern crate rolldice;
    ///
    /// let dice = rolldice::Dice { number: 4, sides: 8 };
    /// let mut rng = rand::thread_rng();
    ///
    /// let result = dice.generate(&mut rng);
    /// ```
    pub fn generate<R: Rng>(&self, mut rng: &mut R) -> u32 {
        let between = Range::new(1, self.sides);
        let mut total = 0;
        for _ in 0..self.number {
            total += between.ind_sample(&mut rng);
        }
        total
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use rand::{SeedableRng, StdRng};

    #[test]
    fn dice_generate() {
        let dice = Dice { sides: 6, number: 4 };

        let seed: &[_] = &[1, 2, 3, 4];
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        let result = dice.generate(&mut rng);
        assert_eq!(result, 10);

        let mut rng = rand::thread_rng();
        for _ in 1..100 {
            let result = dice.generate(&mut rng);
            assert!(result >= 4);
            assert!(result <= 24);
        }
    }

    #[test]
    fn dice_parse_invalid_number_of_dice() {
        assert_eq!(Dice::parse("d6").unwrap_err(), "Invalid number of dice ''");
        assert_eq!(Dice::parse("Td10").unwrap_err(), "Invalid number of dice 'T'");
    }

    #[test]
    fn dice_parse_invalid_number_of_sides() {
        assert_eq!(Dice::parse("1d").unwrap_err(), "Invalid number of sides ''");
        assert_eq!(Dice::parse("1dY").unwrap_err(), "Invalid number of sides 'Y'");
    }

    #[test]
    fn dice_parse_invalid_dice_format() {
        assert_eq!(Dice::parse("").unwrap_err(), "Invalid dice format ''");
        assert_eq!(Dice::parse("something").unwrap_err(), "Invalid dice format 'something'");
    }

    #[test]
    fn dice_parse_correct() {
        assert_eq!(
            Dice::parse("6d100"),
            Ok(Dice { number: 6, sides: 100 }));
        assert_eq!(
            Dice::parse("1d5"),
            Ok(Dice { number: 1, sides: 5}));
    }
}
