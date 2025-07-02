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
        // Push input or handle bad input.
        match Op::from_str(arg) {
            // Operator; push to operator stack.
            Ok(op) => self.ops.push(op),

            // Not a supported operand, check if it's numeric.
            Err(_) => match arg.parse::<f64>() {
                Ok(num) => self.nums.push(num),
                Err(_) => return false,
            },
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
    pub fn get_result(&self) -> Option<f64> {
        if self.nums.len() == 1 && self.ops.len() == 0 {
            Some(self.nums[0])
        } else {
            None
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
