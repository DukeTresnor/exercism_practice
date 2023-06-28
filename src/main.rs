mod string_functions;

use string_functions::*;

fn main() {
    println!("Hello, world!");

    let reverse_hello: String = reverse("Hello, world!");

    println!("Reverse: {}", reverse_hello);
}
