use std::str::FromStr;

/// A unary operator (only takes one input).
#[derive(Clone, Copy)]
pub enum UnOp {
    // Signage:
    Neg,
    Abs,
    Nabs,

    // Square/cube and roots:
    Sqr,
    Sqrt,
    Cub,
    Cbrt,

    // Euler:
    Exp,
    Ln,

    // Trigonometric functions:
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,

    // Hyperbolic trigonometric functions:
    Atan,
    Sinh,
    Cosh,
    Tanh,
}

/// A binary operator (takes two inputs).
#[derive(Clone, Copy)]
pub enum BiOp {
    // Arithmetic:
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    // Powers:
    Pow,
    Log,
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
            UnOp::Neg => -num,
            UnOp::Abs => num.abs(),
            UnOp::Nabs => -num.abs(),
            UnOp::Sqr => num * num,
            UnOp::Sqrt => num.sqrt(),
            UnOp::Cub => num * num * num,
            UnOp::Cbrt => num.cbrt(),
            UnOp::Exp => num.exp(),
            UnOp::Ln => num.ln(),
            UnOp::Sin => num.sin(),
            UnOp::Cos => num.cos(),
            UnOp::Tan => num.tan(),
            UnOp::Asin => num.asin(),
            UnOp::Acos => num.acos(),
            UnOp::Atan => num.atan(),
            UnOp::Sinh => num.sinh(),
            UnOp::Cosh => num.cosh(),
            UnOp::Tanh => num.tanh(),
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
            BiOp::Mod => a % b,
            BiOp::Pow => a.powf(b),
            BiOp::Log => a.ln() / b.ln(),
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
            "neg" => Ok(Op::Un(UnOp::Neg)),
            "abs" => Ok(Op::Un(UnOp::Abs)),
            "nabs" => Ok(Op::Un(UnOp::Nabs)),

            "sqr" => Ok(Op::Un(UnOp::Sqr)),
            "sqrt" => Ok(Op::Un(UnOp::Sqrt)),
            "cub" => Ok(Op::Un(UnOp::Cub)),
            "cbrt" => Ok(Op::Un(UnOp::Cbrt)),

            "exp" => Ok(Op::Un(UnOp::Exp)),
            "ln" => Ok(Op::Un(UnOp::Ln)),

            "sin" => Ok(Op::Un(UnOp::Sin)),
            "cos" => Ok(Op::Un(UnOp::Cos)),
            "tan" => Ok(Op::Un(UnOp::Tan)),
            "asin" => Ok(Op::Un(UnOp::Asin)),
            "acos" => Ok(Op::Un(UnOp::Acos)),
            "atan" => Ok(Op::Un(UnOp::Atan)),
            "sinh" => Ok(Op::Un(UnOp::Sinh)),
            "cosh" => Ok(Op::Un(UnOp::Cosh)),
            "tanh" => Ok(Op::Un(UnOp::Tanh)),

            // Binary operators:
            "+" => Ok(Op::Bi(BiOp::Add)),
            "-" => Ok(Op::Bi(BiOp::Sub)),
            "*" => Ok(Op::Bi(BiOp::Mul)),
            "/" => Ok(Op::Bi(BiOp::Div)),
            "%" => Ok(Op::Bi(BiOp::Mod)),

            "^" => Ok(Op::Bi(BiOp::Pow)),
            "log" => Ok(Op::Bi(BiOp::Log)),

            // Not a supported operand.
            _ => Err(()),
        }
    }
}
