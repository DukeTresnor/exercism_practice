mod string_functions;
mod datetime_functions;
mod math_functions;
mod general_functions;


use std::vec;

use string_functions::*;
use datetime_functions::*;
use math_functions::*;
use general_functions::*;


fn main() {

    let code_test = "4539 3195 0343 6467";
    let validity = is_valid(&code_test);
    println!("validity of code_test {}: {}", code_test, validity);


    let second_test = "1";
    let valid_second = is_valid(&second_test);
    println!("validity of second_test {}: {}", second_test, valid_second);

    /* 
    let num = 23;

    let vectored_number: Vec<_> = num.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    println!("vectored_number: {:?}", vectored_number);
    */


}


fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
