/// An operation, unary or binary.
#[derive(Debug, Clone, Copy)]
pub enum Op<T> {
    Un(fn(T) -> T),
    Bi(fn(T, T) -> T),
}

/// A trait for a calculator mode/format.
pub trait Calc: Sized {
    /// The input types for the calculator's operations.
    type OpType: Clone + Copy + ToString;

    /// Parses the string as a literal.
    fn parse_str(s: &str) -> Option<Self::OpType>;

    /// Maps the input string to a valid numeric if a match is present.
    fn map_lit(s: &str) -> Option<Self::OpType>;

    /// Maps the input string to a valid operation if a match is present.
    fn map_op(s: &str) -> Option<Op<Self::OpType>>;

    /// Runs the calculator.
    fn eval(expr: &str) -> Result<String, String> {
        run_calc::<Self>(expr).map(|n| n.to_string())
    }
}

/// Runs the calculator using the command line arguments.
fn run_calc<T: Calc>(expr: &str) -> Result<T::OpType, String> {
    let mut stack = Vec::new();

    for x in expr.split_ascii_whitespace() {
        if let Some(n) = T::map_lit(x) {
            stack.push(n);
        } else if let Some(op) = T::map_op(x) {
            perform_op(&mut stack, op)?;
        } else {
            return Err(format!("'{x}' is not a valid operator or numeric"));
        }
    }

    match stack.len() {
        1 => Ok(stack[0]),
        _ => Err("expression finished in an incomplete state".to_string()),
    }
}

/// Performs the operation on the calculator's stack.
fn perform_op<T>(stack: &mut Vec<T>, op: Op<T>) -> Result<(), String> {
    match op {
        Op::Un(f) => {
            let n = stack.pop().ok_or("found unary operator with no operands")?;

            stack.push(f(n));
        }
        Op::Bi(f) => {
            let e = || "found binary operator with less than two operands";
            let b = stack.pop().ok_or_else(e)?;
            let a = stack.pop().ok_or_else(e)?;

            stack.push(f(a, b));
        }
    }

    Ok(())
}

/// A macro that simplifies creating a calculator mode.
///
/// For example usage, go see `dec_calc.rs` and/or `bin_calc.rs`.
///
/// # Parameters
/// - The type that you want to implement `Calc` for.
/// - The type that will be stored on the calculator's stack.
/// - The function that is called to parse string literals when no constants
/// match. Should return `None` if invalid.
/// - Pairs of constants, starting with the string literal then the value. For
/// example: `"five" => 5`. The last constant should end with `;`, not `,`.
/// - Pairs of unary operators, starting with the string literal then the
/// operation. For example: `"sqr" => |n| n * n`. The last pair should end with
/// `;`, not `,`.
/// - Pairs of binary operators, starting with the string literal then the
/// operation. For example: `"avg" => |a, b| (a + b) / 2`. The last pair should
/// end with `;`, not `,`.
#[macro_export]
macro_rules! impl_calc {
    ($name:ty, $t:ty, $parse:expr; $($con_name:expr => $con_val:expr),* ; $($un_name:expr => $un_f:expr),* ; $($bi_name:expr => $bi_f:expr),* $(;)*) => {
        use crate::calc::{Calc, Op};

        impl Calc for $name {
            type OpType = $t;

            fn parse_str(s: &str) -> Option<Self::OpType> {
                $parse(s)
            }

            fn map_lit(s: &str) -> Option<Self::OpType> {
                match s {
                    $($con_name => Some($con_val),)*
                    _ => Self::parse_str(s),
                }
            }

            fn map_op(s: &str) -> Option<Op<Self::OpType>> {
                match s {
                    $($un_name => Some(Op::Un($un_f)),)*
                    $($bi_name => Some(Op::Bi($bi_f)),)*
                    _ => None,
                }
            }
        }
    };
}
