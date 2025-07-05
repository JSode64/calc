use std::str::FromStr;

use crate::op::Op;

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
    pub fn process_input(&mut self, arg: &str) -> Result<(), String> {
        if let Ok(op) = Op::from_str(arg) {
            self.process_op(op)?;
        } else {
            self.push_num(arg)?;
        }

        Ok(())
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
    fn process_op(&mut self, op: Op) -> Result<(), String> {
        match op {
            Op::Un(f) => {
                let n = self.pop_num()?;

                self.nums.push(f(n));
            }
            Op::Bi(f) => {
                let b = self.pop_num()?;
                let a = self.pop_num()?;

                self.nums.push(f(a, b));
            }
        }

        Ok(())
    }

    /// Attempts to evaluate and push a numeric value from the input.
    fn push_num(&mut self, arg: &str) -> Result<(), String> {
        use std::f64::consts::{E, PI};

        self.nums.push(match arg {
            // Constants:
            "pi" => PI,
            "e" => E,

            // Assume numeric:
            _ => match arg.parse() {
                Ok(n) => n,
                Err(_) => return Err(format!("Error: '{arg}' is not a valid token")),
            },
        });

        Ok(())
    }

    /// Attempts to pop a number off of the stack.
    fn pop_num(&mut self) -> Result<f64, String> {
        match self.nums.pop() {
            Some(n) => Ok(n),
            None => Err("Error: found an operator with too little arguments".to_string()),
        }
    }
}
