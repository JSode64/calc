mod calc;
mod op;

use calc::Calc;

fn main() {
    use std::env::args;

    let mut calc = Calc::new();

    // Process all inputs.
    for arg in args().skip(1) {
        if !calc.process_input(&arg) {
            println!("Invalid input: {arg}.");
            return;
        }
    }

    // Attempt to print result; tell if input is bad.
    match calc.get_result() {
        Ok(res) => println!("{res}"),
        Err(e) => println!("{e}"),
    }
}
