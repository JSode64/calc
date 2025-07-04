use std::str::FromStr;

use super::op::Op;

/// A Reverse Polish Notation calculator state.
pub struct Calc {
    /// Number stack.
    nums: Vec<f64>,
}

impl Calc {
    /// Returns a new empty calculator state.
    pub const fn new() -> Self {
        Self { nums: Vec::new() }
    }

    /// Processes the given input, numeric or operand.
    pub fn process_input(&mut self, arg: &str) -> Option<String> {
        if let Ok(op) = Op::from_str(arg) {
            self.process_op(op)?;
        } else {
            self.push_num(arg)?;
        }

        None
    }

    /// Attempts to return the calculator's result.
    pub fn get_result(&self) -> Option<f64> {
        if self.nums.len() == 1 {
            Some(self.nums[0])
        } else {
            None
        }
    }

    /// Attempts to preform the given operator to the stack.
    fn process_op(&mut self, op: Op) -> Option<String> {
        match op {
            Op::Un(op) => match self.nums.last_mut() {
                Some(n) => *n = op.apply(*n),
                None => {
                    return Some(
                        "Invalid expression: found unary operator with no arguments.".to_string(),
                    );
                }
            },
            Op::Bi(op) => {
                let b = self.nums.pop();
                let a = self.nums.pop();

                match (a, b) {
                    (Some(a), Some(b)) => self.nums.push(op.apply(a, b)),
                    _ => return Some(
                        "Invalid expression: found binary operator with less than two arguments."
                            .to_string(),
                    ),
                }
            }
        }

        // Success.
        None
    }

    /// Attempts to evaluate and push a numeric value from the input.
    fn push_num(&mut self, arg: &str) -> Option<String> {
        use std::f64::consts::{E, PI};

        self.nums.push(match arg {
            // Constants:
            "pi" => PI,
            "e" => E,

            // Assume numeric:
            _ => match arg.parse() {
                Ok(n) => n,
                Err(_) => {
                    return Some(format!(
                        "Invalid input: '{arg}' is not an operator or numeric."
                    ));
                }
            },
        });

        // Success.
        None
    }
}
