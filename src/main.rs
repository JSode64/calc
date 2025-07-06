mod bin_calc;
mod calc;
mod dec_calc;

fn main() -> Result<(), String> {
    use bin_calc::BinCalc;
    use calc::Calc;
    use dec_calc::DecCalc;
    use std::env::args;

    match args().nth(1) {
        None => return Err("no arguments given".to_string()),
        Some(x) => println!(
            "{}",
            match x.as_str() {
                "-b" => BinCalc::run()?.to_string(),
                _ => DecCalc::run()?.to_string(),
            }
        ),
    }

    Ok(())
}
