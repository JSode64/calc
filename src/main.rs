mod bin_calc;
mod calc;
mod dec_calc;

/// Used to track how the next input argument should be handled.
#[derive(Clone, Copy)]
enum Mode {
    Bin,
    Dec,
}

fn main() -> Result<(), String> {
    use bin_calc::BinCalc;
    use calc::Calc;
    use dec_calc::DecCalc;
    use std::env::args;

    let mut mode = None;

    for arg in args().skip(1) {
        if let Some(m) = mode {
            // Feed the input into its chosen calculator.
            println!(
                "{}",
                match m {
                    Mode::Bin => BinCalc::eval(&arg)?,
                    Mode::Dec => DecCalc::eval(&arg)?,
                }
            );
            mode = None;
        } else {
            match arg.as_str() {
                "-b" => mode = Some(Mode::Bin),
                "-d" => mode = Some(Mode::Dec),

                // No valid mode selected.
                _ => return Err(format!("expected '-b' or '-d', found '{arg}'")),
            }
        }
    }

    Ok(())
}
