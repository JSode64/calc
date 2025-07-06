use crate::impl_calc;

use std::{
    ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Not, Rem, Shl, Shr, Sub},
    u64::MAX,
};

/// Parses the string into a `u64`, using codes to provide base.
fn parse_str(s: &str) -> Option<u64> {
    let (base, start) = match s.as_bytes()[0] {
        b'b' => (2, 1),
        b'o' => (8, 1),
        b'd' => (10, 1),
        b'x' => (16, 1),
        _ => (10, 0),
    };

    u64::from_str_radix(&s[start..], base).ok()
}

/// A 64-bit binary calculator.
pub struct BinCalc;

impl_calc!(
    BinCalc,
    u64,
    parse_str,
    2;

    // Constants start:

    "MAX" => MAX;

    // Unary operators start:

    "!" => |n| (n == 0) as _,
    "?" => |n| (n != 0) as _,

    "~" => u64::not;

    // Binary operators start:

    "+" => u64::add,
    "-" => u64::sub,
    "*" => u64::mul,
    "/" => u64::div,
    "%" => u64::rem,

    "&" => u64::bitand,
    "|" => u64::bitor,
    "^" => u64::bitxor,
    "!&" => |a, b| !(a & b),
    "!|" => |a, b| !(a | b),
    "!^" => |a, b| !(a ^ b),

    "<<" => u64::shl,
    ">>" => u64::shr,

    ">" => |a, b| (a > b) as _,
    "<" => |a, b| (a < b) as _,
    "==" => |a, b| (a == b) as _,
    "!=" => |a, b| (a != b) as _,

    "min" => u64::min,
    "max" => u64::max;
);
