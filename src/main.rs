extern crate clap;
extern crate rand;
extern crate rolldice;

use rolldice::Dice;
use clap::{Arg, App};


fn main() {
    let matches = App::new("rolldice")
        .arg(Arg::with_name("dice")
             .required(true))
        .get_matches();

    let dice = matches.value_of("dice").unwrap();
    let dice = Dice::parse(dice).unwrap();

    let mut rng = rand::thread_rng();

    println!("{:?}", dice.generate(&mut rng));
}
