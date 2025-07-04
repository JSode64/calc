//! # Operator System
//!
//! This file defines operators and how they work under the hood.
//! There are two macros that do everything: `make_un_op` and `make_bi_op`.
//!
//! To add more operators, simply put them in their macro.
//!
//! A unary operator is an operator with one argument, like `abs` or `sqrt`.
//! A binary operator has two arguments, like adding and subtracting.

use std::str::FromStr;

/// An operator; unary or binary.
#[derive(Debug, Clone, Copy)]
pub enum Op {
    Un(UnOp),
    Bi(BiOp),
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        UnOp::from_str(s)
            .map(Self::Un)
            .or_else(|_| BiOp::from_str(s).map(Self::Bi))
    }
}

macro_rules! make_bi_op {
    ($name:ident, $($op:ident, $s:expr, $f:expr);* $(;)*) => {
        #[derive(Debug, Clone, Copy)]
        pub enum $name {
            $($op),*
        }

        impl $name {
            /// Applies the operator to the two given operands. Returns the result.
            pub fn apply(self, a: f64, b: f64) -> f64 {
                match self {
                    $(Self::$op => $f(a, b)),*
                }
            }
        }

        impl FromStr for $name {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($s => Ok(Self::$op),)*
                    _ => Err(()),
                }
            }
        }
    };
}

macro_rules! make_un_op {
    ($name:ident, $($op:ident, $s:expr, $f:expr);* $(;)*) => {
        #[derive(Debug, Clone, Copy)]
        pub enum $name {
            $($op),*
        }

        impl $name {
            /// Applies the operator to the two given operands. Returns the result.
            pub fn apply(self, n: f64) -> f64 {
                match self {
                    $(Self::$op => $f(n)),*
                }
            }
        }

        impl FromStr for $name {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($s => Ok(Self::$op),)*
                    _ => Err(()),
                }
            }
        }
    };
}

make_bi_op!(
    BiOp,

    Add, "+", |a, b| a + b;
    Sub, "-", |a, b| a - b;
    Mul, "*", |a, b| a * b;
    Div, "/", |a, b| a / b;
    Mod, "%", |a, b| a % b;

    Pow, "^", |a: f64, b| a.powf(b);
    Log, "log", |a: f64, b| a.log(b);

    Max, "max", |a: f64, b| a.max(b);
    Min, "min", |a: f64, b| a.min(b);
);

make_un_op!(
    UnOp,

    Neg, "neg", |n: f64| -n;
    Abs, "abs", |n: f64| n.abs();
    Nabs, "nabs", |n: f64| -n.abs();

    Sqr, "sqr", |n: f64| n * n;
    Sqrt, "sqrt", |n: f64| n.sqrt();
    Cub, "cub", |n: f64| n * n * n;
    Cbrt, "cbrt", |n: f64| n.cbrt();

    Exp, "exp", |n: f64| n.exp();
    Ln, "ln", |n: f64| n.ln();

    Sin, "sin", |n: f64| n.sin();
    Cos, "cos", |n: f64| n.cos();
    Tan, "tan", |n: f64| n.tan();
    Asin, "asin", |n: f64| n.asin();
    Acos, "acos", |n: f64| n.acos();
    Atan, "atan", |n: f64| n.atan();

    Sind, "sind", |n: f64| n.to_radians().sin();
    Cosd, "cosd", |n: f64| n.to_radians().cos();
    Tand, "tand", |n: f64| n.to_radians().tan();
    Asind, "asind", |n: f64| n.to_radians().asin();
    Acosd, "acosd", |n: f64| n.to_radians().acos();
    Atand, "atand", |n: f64| n.to_radians().atan();

    Sinh, "sinh", |n: f64| n.sinh();
    Cosh, "cosh", |n: f64| n.cosh();
    Tanh, "tanh", |n: f64| n.tanh();
);
