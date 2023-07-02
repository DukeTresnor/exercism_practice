// math_functions / mod.rs

use std::num::*;

use crate::print_type_of;

pub fn is_armstrong_number(num: u32) -> bool {

    // converts number to string, string to chars, then
    //   maps each character c into a digit with base 10 with to_digit(10),
    //   then unwraps the option, then collects the results
    let vectored_number: Vec<_> = num.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let mut potential_armstrong_number = 0;
    // Loops through vectored_number
    for digit in vectored_number.iter() {
        // Checks to see if digit^vectored_number[i] overflows or not
        if num::checked_pow(*digit, vectored_number.len()).is_some() {
            let added_value = num::checked_pow(*digit, vectored_number.len()).unwrap();
            // If add_result is some u32, add your old guess for armstrong numbers to your current added_value. Otherwise, there's an overflow,
            //   so return an error statement
            let add_result = add_numbers(Some(potential_armstrong_number), Some(added_value));
            match add_result {
                Some(_sum) => potential_armstrong_number += added_value,
                None => println!("Can't add the two numbers"),
            }
        } else {
            potential_armstrong_number += 0;
        }
        
    }

    if potential_armstrong_number == num.try_into().unwrap() {
        return true
    } else {
        return false
    }

    
}

// Adds two numbers together by sending in those numbers as Options, adding them together, and checking for overflow along with them being Some number
pub fn add_numbers(a: Option<u32>, b: Option<u32>) -> Option<u32> {
    match (a, b) {
        (Some(num1), Some(num2)) => num1.checked_add(num2),
        _ => None,
    }
}