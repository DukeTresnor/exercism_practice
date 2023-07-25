mod string_functions;
mod datetime_functions;
mod math_functions;
mod general_functions;


use std::vec;

use string_functions::*;
use datetime_functions::*;
use math_functions::*;
use general_functions::*;


//use math_functions::Planet::{Mars, Venus};
//crate::math_functions::{Mercury, Venus, Earth, Mars, Jupiter, Saturn, Uranus, Neptune};

fn main() {

    let test_duration = Duration {
        duration: 2_129_871_239
    };

    println!("test_duration: {}", test_duration.duration);

    let test_planet = Mars;
    //println!("testing: {}", Mars::years_during(&test_duration));

    println!("years on test_planet: {}", test_planet.years_during(&test_duration));
    //test_planet.years_during(&test_duration);

}


fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
