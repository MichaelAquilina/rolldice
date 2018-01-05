extern crate clap;
extern crate rand;
extern crate rolldice;

use rolldice::Dice;
use clap::{Arg, App};


fn main() {
    let matches = App::new("rolldice")
        .arg(Arg::with_name("dice")
             .required(true)
             .multiple(true))
        .get_matches();

    let mut rng = rand::thread_rng();
    for dice in matches.values_of("dice").unwrap() {
        let dice = Dice::parse(dice).unwrap();

        print!("{} ", dice.generate(&mut rng));
    }
    println!("");
}
