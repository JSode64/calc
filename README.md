# Description

This is a simple RPN (Reverse Polish Notation) command-line calculator.
It provides basic unary and binary operations, as well as some constants.

It has two modes: decimal and binary. Decimal is a regular base 10 calculator with `f64`s, while binary allows boolean algebra operators, boolean expressions/comparisons, and support for base 2, 8, 10, and 16 values via prefixes (all use `u64`).

The modes can be chosen by using "-b" for binary or "-d" for decimal before the expression.

# Decimal Examples

Adding 1 and 2:
```
>> calc -d "1 2 +"
3
```

Using constants (pi times e):
```
>> calc -d "PI E *"
8.539734222673566
```
Larger calls:
```
>> calc -d "4 2 2 * * 2 + 10 log sqrt sin nabs"
-0.9002700476109888
```

# Binary Examples

Note: To access the binary calculator, start the expression with `-b`.

Bitwise-ORing 3 and 6:
```
calc -b "3 6 |"
7
```

Seeing if 5 is larger than 6:
```
calc -b "5 6 >"
0
```

Simple boolean expression:
```
calc -b "T F | T &"
1
```

Larger calls:
```
calc -b "MAX 5 - MAX 32 >> !& 3 5 << ^"
18446744069414584421
```

# Multiple Calls Example

Evaluating multiple expressions in one run:
```
calc -d "2 2 +" -b "1 3 <<" -b "3 1 &" -d "2 sqrt nabs"
4
8
1
-1.4142135623730951
```

---

### Notes

- Operators can *not* be given ahead of time, meaning expressions such as `+ 1 2` will not work. Do `1 2 +` instead.
- To add more operators, see `dec_calc` for the decimal calculator or `bin_calc.rs` for the binary calculator.
- To add a new calculator mode, view `impl_calc` in `calc.rs` to see how to make one yourself.