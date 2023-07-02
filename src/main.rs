mod string_functions;
mod datetime_functions;
mod math_functions;


use string_functions::*;
use datetime_functions::*;
use math_functions::*;


fn main() {


    let techer: u32 = 3548299154;
    let becher = 1073741824;


    let techerbecher = math_functions::add_numbers(Some(techer), Some(becher));
    match techerbecher {
        Some(sum) => println!("Sum: {}", sum),
        None => println!("One or both of the numbers are missing."),
    }

    let booler = is_armstrong_number(4_106_098_957);

    // 9474
    //4_106_098_957

    println!("booler: {}", booler);


}


fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
