// math_functions / mod.rs

use std::{num::*, cell};

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


// -- Conversion Functions -- // 

/*
    let vectored_number: Vec<_> = num.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

*/

// -- Conversion Functions -- // 



// -- Squares of Series Functions -- //

pub fn square_of_sum(n: u32) -> u32 {
    // Using formula for the square of the sum of natural numbers
    // Sum of natural numbers -- sum of n from 1 to n is ( n * (n + 1) ) / 2
    let sum = ( n * (n + 1) ) / 2;
    sum * sum
}

pub fn sum_of_squares(n: u32) -> u32 {
    // Using formula for the sum of squares -- sum of n^2 from 1 to n is n * (n + 1) * (2n + 1) / 6
    n * (n + 1) * (2 * n + 1) / 6
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}

// -- Squares of Series Functions -- //





// -- Chess Square Functions -- //

pub fn square(s: u32) -> u64 {
    
    let mut grain_amount = 1;    
    if is_chess_square(&s) {
        for _ in 0..s-1 {
            grain_amount *= 2;
        }
    }
    grain_amount
}

pub fn total() -> u64 {
    let mut total_grain = 0;
    for cell_number in 1..65 {
        total_grain += square(cell_number);
        println!("cell number:{}", cell_number);
    }
    total_grain
}

// Issue, I should try to have this return an option?
//  Problem might require a panic message specifically...
pub fn is_chess_square(square: &u32) -> bool {
    if *square < 1 || *square > 64 {
        panic!("Square must be between 1 and 64");
    }
    true
}

// -- Chess Square Functions -- //





// -- Luhn checksum -- Functions -- //

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // params: code -- string slice representing the input identification number
    // returns: is_valid_credit_number -- boolean that is true if the input is a valid identification number
    let mut is_valid_credit_number: bool = true;

    // -- stripping code input -- //
    // Strings of length 1 or less are not valid.
    // Spaces are allowed in the input, but they should be stripped before checking.
    // All other non-digit characters are disallowed. -> no letters or other characters
    // Converting code input into a vector of numbers

    println!("code length: {}", code.len());

    //if code.len() <= 1 {
    //    is_valid_credit_number = false;
    //}

    let snipped_code = code.replace(" ", "");

    if snipped_code.len() <= 1 {
        is_valid_credit_number = false;
    }

    let is_snipped_code_numeric: Vec<bool> = snipped_code.chars().map(|c| c.is_numeric() ).collect();

    // Does is_snipped_code_numeric contain any false bools for its values? If so, we have an invalid input, return false
    if is_snipped_code_numeric.contains(&false) {
        return false;
    }


    // replace the unrawp() command here with a match case Some(value) => {continue?} or None => {return false}
    //let vectored_code: Vec<_> = match snipped_code {
    //    Some(code) => code.chars().map(|c| c.to_digit(10).unwrap()).collect(),
    //    None       => return false,
    //};
    
    
    //println!("vectored_code vec: {:?}\n", vectored_code);
    //let vectored_code: Vec<_> = snipped_code.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let vectored_code: Vec<_> = snipped_code.chars().map(|c| 
        match c.to_digit(10) {
            Some(c) => c,
            None => {
                is_valid_credit_number = false;
                0 as u32
            },            
        }
    
    ).collect();


    // -- stripping code input -- //


    
    // -- checking code validity -- //
    //  double every second digit, starting from the right.
    // If doubling the number results in a number greater than 9 then subtract 9 from the product.
    // Then sum all of the digits
    // If the sum is evenly divisible by 10, then the number is valid




    let mut digit_sum = 0;

    for (i, mut digit) in (*vectored_code).iter().rev().enumerate() {
        
        if i%2 != 0 {
            //println!("digit from end: {}", digit);
            let double_digit = &(*digit * 2);
            println!("digit from end: {}", double_digit);
            if *double_digit >= 10 {
                //digit = &(*double_digit - 9);
                digit_sum = digit_sum + (*double_digit - 9);
            } else {
                //digit = double_digit;
                digit_sum = digit_sum + *double_digit;
            }
            
        } else {
            digit_sum = digit_sum + *digit;
        }

    }

    println!("digit_sum: {}", digit_sum);

    if digit_sum%10 != 0 {
        is_valid_credit_number = false;
    }



    // -- checking code validity -- //

    is_valid_credit_number
}



// -- Luhn checksum -- Functions -- //