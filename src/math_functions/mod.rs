// math_functions / mod.rs

use std::num::*;

pub fn is_armstrong_number(num: u32) -> bool {

    // converts number to string, string to chars, then
    //   maps each character c into a digit with base 10 with to_digit(10),
    //   then unwraps the option, then collects the results
    let vectored_number: Vec<_> = num.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    println!("vectored_number: {:?}", vectored_number);


    let mut potential_armstrong_number = 0;
    for digit in vectored_number.iter() {
        println!("digit: {}", *digit);
        println!("overflow test: {:?}", num::checked_pow(*digit, vectored_number.len()).unwrap() );
        println!("length of vector: {}", vectored_number.len());
        //if u32::overflowing_pow(*digit, vectored_number.len().try_into().unwrap()).1 {
        if num::checked_pow(*digit, vectored_number.len()).is_some() {
            //potential_armstrong_number += u32::overflowing_pow(*digit, vectored_number.len().try_into().unwrap()).0;
            potential_armstrong_number += num::checked_pow(*digit, vectored_number.len()).unwrap();
            // issue is that we get an overflow error when we add, not in the power calculation itself
        } else {
            // the power is None
            // what to do here?
            potential_armstrong_number += 0;
        }
        
    }

    println!("Original number: {}", num);
    println!("the potential_armstrong_number: {}", potential_armstrong_number);

    if potential_armstrong_number == num.try_into().unwrap() {
        return true
    } else {
        return false
    }

    
}
