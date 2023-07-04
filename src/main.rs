mod string_functions;
mod datetime_functions;
mod math_functions;


use string_functions::*;
use datetime_functions::*;
use math_functions::*;


fn main() {

    let chess_square: u32 = 64;

    let output = square(chess_square);

    let grain_total = total();

    println!("grains on chess_square {}: {}", chess_square, output);
    println!("The total is: {}", grain_total);

    let my_num = grain_total as f64 / 18446744073709551615.0;

    // 9223372036854775807

    // should be 18446744073709551615

    // target result is bigger by factor of 2 -- meaning that i am missing one instance of doubling

    println!("my_num: {}", my_num);


}


fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
