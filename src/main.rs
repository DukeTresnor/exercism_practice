mod string_functions;
mod datetime_functions;
mod math_functions;


use string_functions::*;
use datetime_functions::*;
use math_functions::*;


fn main() {


    let booler = is_armstrong_number(4_106_098_957);

    // 9474
    //4_106_098_957

    println!("booler: {}", booler);


}
