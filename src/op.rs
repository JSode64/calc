use std::str::FromStr;

/// A unary operator (only takes one input).
#[derive(Clone, Copy)]
pub enum UnOp {
    Abs,
    Sqr,
    Sqrt,
}

/// A binary operator (takes two inputs).
#[derive(Clone, Copy)]
pub enum BiOp {
    Add,
    Sub,
    Mul,
    Div,
}

/// An operator, unary or binary.
#[derive(Clone, Copy)]
pub enum Op {
    Un(UnOp),
    Bi(BiOp),
}

impl UnOp {
    /// Returns the operation done on the operand.
    pub fn apply(self, num: f64) -> f64 {
        match self {
            UnOp::Abs => num.abs(),
            UnOp::Sqr => num * num,
            UnOp::Sqrt => num.sqrt(),
        }
    }
}

impl BiOp {
    /// Returns the operation done on the operands.
    pub fn apply(self, a: f64, b: f64) -> f64 {
        match self {
            BiOp::Add => a + b,
            BiOp::Sub => a - b,
            BiOp::Mul => a * b,
            BiOp::Div => a / b,
        }
    }
}

impl Op {
    /// Returns the number of operands needed for the operator.
    pub fn num_operands(self) -> usize {
        match self {
            Op::Un(_) => 1,
            Op::Bi(_) => 2,
        }
    }
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // Unary operators:
            "abs" => Ok(Op::Un(UnOp::Abs)),
            "sqr" => Ok(Op::Un(UnOp::Sqr)),
            "sqrt" => Ok(Op::Un(UnOp::Sqrt)),

            // Binary opeerators:
            "+" => Ok(Op::Bi(BiOp::Add)),
            "-" => Ok(Op::Bi(BiOp::Sub)),
            "*" => Ok(Op::Bi(BiOp::Mul)),
            "/" => Ok(Op::Bi(BiOp::Div)),

            // Not a supported operand.
            _ => Err(()),
        }
    }
}
