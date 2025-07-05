# Description

This is a simple RPN (Reverse Polish Notation) command-line calculator.
It provides basic unary and binary operations, as well as some constants.

# Examples

Adding 1 and 2:
```
>> calc 1 2 +
3
```

Using constants (pi times e):
```
>> calc pi e *
8.539734222673566
```
Larger calls:
```
>> calc 2 sqr sqr 2 + 10 log sqrt sin nabs
-0.9002700476109888
```

---

### Notes

- Operators can *not* be given ahead of time, meaning expressions such as `+ 1 2` will not work. Do `1 2 +` instead.
- To add more operators, see `op.rs` and read the instructions at the top.