extern crate clap;
extern crate rand;
extern crate rolldice;

use rolldice::Dice;
use clap::{Arg, App};


fn main() {
    let matches = App::new("rolldice")
        .arg(Arg::with_name("dice")
             .help("Specify dice to roll in the form of <number>d<sides> e.g. 2d6")
             .required(true)
             .multiple(true))
        .get_matches();

    let mut rng = rand::thread_rng();
    for dice in matches.values_of("dice").unwrap() {
        match Dice::parse(dice) {
            Ok(v) => print!("{} ", v.generate(&mut rng)),
            Err(message) => print!("{}", message),
        }
    }
    println!("");
}
