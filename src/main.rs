mod string_functions;
mod datetime_functions;


use string_functions::*;
use datetime_functions::*;


fn main() {
    println!("Hello, world!");

    let reverse_hello: String = reverse("Hello, world!");

    println!("Reverse: {}", reverse_hello);

    let modtry = 10000 % 46;
    let divtry = 10000 / 46;

    println!("modtry: {}, divtry: {}", modtry, divtry);


}
