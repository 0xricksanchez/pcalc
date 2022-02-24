# pcalc
pcalc clone written in rust for the sole purpose of it being able to handle 64-bit numbers. 
It's happily doing arithmetics with overflows just wrapping around the `usize` space.

## Usage

![image](https://user-images.githubusercontent.com/17012133/155534296-3dd1dd85-52f2-4392-8eff-a12f15cf8698.png)

## Implemented Operators

- Addition ⇾ "+"
- Subtraction ⇾ "-"
- Multiplication ⇾ "*"
- Division ⇾ "/"
- Modulo ⇾ "%"
- Right shift ⇾ ">>"
- Left shift ⇾ "<<"
- Bitwise or ⇾ "||"
- Bitwise and ⇾ "&&"

## Known issues

- Built-in bash operators like "*" or ">>" requires putting quotes around the expressions
- Only two operands are supported, this is not intended to handle arbitrary large expressions
