use hello_macro::Hello_Macro;
use hello_macro_derive::Hello_Macro;

fn main() {
    println!("Hello, world!");
}

#[derive(Hello_Macro)]
struct Pancakes;
fn main() {
    Pancakes::hello_macro();
}
