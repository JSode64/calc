use crate::impl_calc;

use std::f64::consts::{E, PI};
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

/// A decimal calculator.
pub struct DecCalc;

impl_calc!(
    DecCalc,
    f64,
    |s: &str| s.parse().ok(),
    1;

    // Constants start:

    "pi" => PI,
    "e" => E;

    // Unary operators start:

    "neg" => f64::neg,
    "abs" => f64::abs,
    "nabs" => |n| -n.abs(),

    "inv" => |n| 1.0 / n,
    "sqrt" => f64::sqrt,
    "cbrt" => f64::cbrt,

    "exp" => f64::exp,
    "ln" => f64::ln,

    "sin" => f64::sin,
    "cos" => f64::cos,
    "tan" => f64::tan,
    "asin" => f64::asin,
    "acos" => f64::acos,
    "atan" => f64::atan,

    "sind" => |n| n.to_radians().sin(),
    "cosd" => |n| n.to_radians().cos(),
    "tand" => |n| n.to_radians().tan(),
    "asind" => |n| n.to_radians().asin(),
    "acosd" => |n| n.to_radians().acos(),
    "atand" => |n| n.to_radians().atan(),

    "sinh" => f64::sinh,
    "cosh" => f64::cosh,
    "tanh" => f64::tanh,
    "asinh" => f64::asinh,
    "acosh" => f64::acosh,
    "atanh" => f64::atanh;

    // Binary operations start:

    "+" => f64::add,
    "-" => f64::sub,
    "*" => f64::mul,
    "/" => f64::div,
    "%" => f64::rem,

    "root" => |a, b| a.powf(1.0 / b),
    "pow" => f64::powf,
    "log" => f64::log,

    "max" => f64::max,
    "min" => f64::min;
);
