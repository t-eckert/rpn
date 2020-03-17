# Goals for Streams

[Streaming on Twitch](https://www.twitch.tv/arbornor)

## Goals 00

2020-03-16

Have a function that takes a string of operators and operands in RPN and evaluates them.

Example: `3 4 +` instead of "infix" notation `3 + 4`

We should take operands and add them to a stack. Operators pop two operands off of the stack and evaluate them.

Tests:

- `3 4 +` == `7`
- `4 3 -` == `1`
