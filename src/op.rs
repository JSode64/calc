//! # Operator System
//!
//! All operators are defined in a macro at the bottom of the file.
//! To add more, just give the input token string, its kind, and the function.
//!
//! The different kinds are `Op::Bi` and `Op::Un`, representing binary and
//! unary operators respectively.
//! Binary operators take two inputs, for example adding two numbers.
//! Unary operators take one input, for examples getting the square root of a number.

use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
use std::str::FromStr;

/// An operator; unary or binary.
#[derive(Debug, Clone, Copy)]
pub enum Op {
    Un(fn(f64) -> f64),
    Bi(fn(f64, f64) -> f64),
}

macro_rules! make_ops {
    ($($name:expr, $kind:expr, $f:expr);* $(;)?) => {
        impl FromStr for Op {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($name => Ok($kind($f)),)*
                    _ => Err(())
                }
            }
        }
    };
}

make_ops!(
    "+", Op::Bi, f64::add;
    "-", Op::Bi, f64::sub;
    "*", Op::Bi, f64::mul;
    "/", Op::Bi, f64::div;
    "%", Op::Bi, f64::rem;

    "root", Op::Bi, |a, b| a.powf(1.0 / b);
    "pow", Op::Bi, f64::powf;
    "log", Op::Bi, f64::log;

    "max", Op::Bi, f64::max;
    "min", Op::Bi, f64::min;

    "neg", Op::Un, f64::neg;
    "abs", Op::Un, f64::abs;
    "nabs", Op::Un, |n| -n.abs();

    "inv", Op::Un, |n| 1.0 / n;
    "sqrt", Op::Un, f64::sqrt;
    "cbrt", Op::Un, f64::cbrt;

    "exp", Op::Un, f64::exp;
    "ln", Op::Un, f64::ln;

    "sin", Op::Un, f64::sin;
    "cos", Op::Un, f64::cos;
    "tan", Op::Un, f64::tan;
    "asin", Op::Un, f64::asin;
    "acos", Op::Un, f64::acos;
    "atan", Op::Un, f64::atan;

    "sind", Op::Un, |n| n.to_radians().sin();
    "cosd", Op::Un, |n| n.to_radians().cos();
    "tand", Op::Un, |n| n.to_radians().tan();
    "asind", Op::Un, |n| n.to_radians().asin();
    "acosd", Op::Un, |n| n.to_radians().acos();
    "atand", Op::Un, |n| n.to_radians().atan();

    "sinh", Op::Un, f64::sinh;
    "cosh", Op::Un, f64::cosh;
    "tanh", Op::Un, f64::tanh;
    "asinh", Op::Un, f64::asinh;
    "acosh", Op::Un, f64::acosh;
    "atanh", Op::Un, f64::atanh;
);
