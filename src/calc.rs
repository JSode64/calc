use std::str::FromStr;

use super::op::Op;

/// A Reverse Polish Notation calculator state.
pub struct Calc {
    /// Number stack.
    nums: Vec<f64>,

    /// Operator stack.
    ops: Vec<Op>,
}

impl Calc {
    /// Returns a new empty calculator state.
    pub const fn new() -> Self {
        Self {
            nums: Vec::new(),
            ops: Vec::new(),
        }
    }

    /// Processes the given input, numeric or operand.
    /// Returns `true` on success, `false` if the input is invalid.
    pub fn process_input(&mut self, arg: &str) -> bool {
        use std::f64::consts::{E, PI};

        // Push input or handle bad input.
        if let Ok(op) = Op::from_str(arg) {
            self.ops.push(op);
        } else {
            self.nums.push(match arg {
                // Contants:
                "pi" => PI,
                "2pi" => PI * 2.0,
                "e" => E,

                // Check for literal.
                _ => match arg.parse() {
                    Ok(num) => num,
                    Err(_) => return false,
                },
            });
        }

        // Apply/process if possible.
        if self.can_apply() {
            // Match and perform operation.
            let op = self.ops.pop().unwrap();

            match op {
                Op::Un(op) => {
                    let n = self.nums.last_mut().unwrap();
                    *n = op.apply(*n);
                }
                Op::Bi(op) => {
                    let b = self.nums.pop().unwrap();
                    let a = self.nums.pop().unwrap();

                    self.nums.push(op.apply(a, b));
                }
            }
        }

        true
    }

    /// Attempts to return the calculator's result.
    /// If the calculator is not in a valid result state, returns `None`. If it
    /// is, returns the result.
    pub fn get_result(&self) -> Result<f64, &str> {
        if self.nums.len() != 1 {
            Err("Too many numbers.")
        } else if !self.ops.is_empty() {
            Err("Too many operators.")
        } else {
            Ok(self.nums[0])
        }
    }

    /// Returns true if it is safe to apply an operator.
    ///
    /// To apply an operator, there must be:
    /// - At least one operator in the operator stack
    /// - A sufficient amount of operands/numbers to apply the top operator
    fn can_apply(&self) -> bool {
        self.ops
            .last()
            .is_some_and(|op| self.nums.len() >= op.num_operands())
    }
}
