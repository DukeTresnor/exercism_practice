mod string_functions;
mod datetime_functions;
mod math_functions;
mod general_functions;


use string_functions::*;
use datetime_functions::*;
use math_functions::*;
use general_functions::*;


fn main() {

    let word = "Orchestra";
    let inputs = ["cashregister", "Carthorse", "radishes", "orchestra"];
    let outputs = vec!["Carthorse"];

    let junk = anagrams_for(word, &inputs);

    println!("junk: {:?}", junk);

    let mut v = [-5, 4, 1, -3, 2];

    v.sort_unstable();
    assert!(v == [-5, -3, 1, 2, 4]);

    println!("v sorted: {:?}", v);



    let mut word_vec: Vec<_> = word.chars().collect();
    word_vec.sort_unstable();
    for letter in word_vec {
        println!("letter: {}", letter);
    }

}


fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
