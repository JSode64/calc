mod calc;
mod op;

fn main() {
    use calc::Calc;
    use std::env::args;

    let mut calc = Calc::new();

    // Process all inputs.
    for arg in args().skip(1) {
        if let Err(e) = calc.process_input(&arg) {
            println!("{e}");
            return;
        }
    }

    // Attempt to print result; tell if input is bad.
    match calc.get_result() {
        Some(n) => println!("{n}"),
        None => println!("Invalid expression: didn't end with a finished state"),
    }
}
