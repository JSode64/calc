mod calc;
mod op;

use calc::Calc;

fn main() {
    use std::env::args;

    let mut calc = Calc::new();

    for arg in args().skip(1) {
        if !calc.process_input(&arg) {
            println!("Error.");
            return;
        }
    }

    match calc.get_result() {
        Some(res) => println!("{res}"),
        None => println!("Invalid input."),
    }
}
