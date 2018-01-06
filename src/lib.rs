extern crate rand;

use rand:: Rng;
use rand::distributions::{Range, IndependentSample};


#[derive(Debug, PartialEq, Eq)]
pub struct Dice {
    pub number: u32,
    pub sides: u32,
}

impl Dice {
    /// Create a Dict instance give a standard dice format.
    /// Expected format is <number>d<sides> where <number>
    /// is the number of dice to roll and <sides> is the number
    /// of sides per dice rolled.
    ///
    /// ```
    /// let result = rolldice::Dice::parse("4d6").unwrap();
    ///
    /// assert_eq!(result, rolldice::Dice { number: 4, sides: 6 });
    /// ```
    pub fn parse(dice: &str) -> Result<Dice, &str> {
        let tokens: Vec<&str> = dice.split("d").collect();

        if tokens.len() != 2 {
            return Err("Invalid format");
        }

        let number = match tokens[0].parse() {
            Ok(num) => num,
            Err(_) => return Err("Expected valid number of dice"),
        };
        let sides = match tokens[1].parse() {
            Ok(num) => num,
            Err(_) => return Err("Expected valid number of sides"),
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
    fn dice_parse_none() {
        assert!(Dice::parse("d6").is_err());
        assert!(Dice::parse("1d").is_err());
        assert!(Dice::parse("").is_err());
        assert!(Dice::parse("something").is_err());
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
