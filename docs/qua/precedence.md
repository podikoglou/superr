# Qua Operator Precedence

Taken from [Crafting Interpreters](https://www.craftinginterpreters.com/parsing-expressions.html)
(itself [taken from C](https://en.cppreference.com/w/c/language/operator_precedence))

From *lowest*, to *highest* precedence .

| Name       | Operators | Associates |
| ---------- | --------- | ---------- |
| Equality   | == !=     | Left       |
| Comparison | > >= < <= | Left       |
| Term       | - +       | Left       |
| Factor     | / \*      | Left       |
| Unary      | ! -       | Right      |
