use std::{cmp::Ordering, env};

const HEX_PREFIX: &str = "0x";

enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
    BitwiseAnd,
    BitwiseOr,
    ShiftRight,
    ShiftLeft,
}

struct Math {
    lhs: usize,
    op: Operation,
    rhs: usize,
    res: usize,
}

impl Math {
    fn check_num(num_str: &str) -> (&str, bool) {
        let tr = if num_str.starts_with(HEX_PREFIX) {
            num_str.trim_start_matches(HEX_PREFIX).trim()
        } else {
            num_str.trim()
        };
        if tr.chars().all(|x| x.is_ascii_digit()) {
            (tr, false)
        } else if tr.chars().all(|x| x.is_ascii_hexdigit()) {
            (tr, true)
        } else {
            eprintln!("Failed to parse one of the inputs!");
            std::process::exit(-1);
        }
    }

    fn hexstr_to_int(inp: &str, is_hex: bool) -> usize {
        if is_hex {
            usize::from_str_radix(inp, 16).unwrap()
        } else {
            inp.parse::<usize>().unwrap()
        }
    }

    fn get_op(op_str: &str) -> Option<Operation> {
        match op_str {
            "+" => Some(Operation::Addition),
            "-" => Some(Operation::Subtraction),
            "*" => Some(Operation::Multiplication),
            "/" => Some(Operation::Division),
            "%" => Some(Operation::Modulo),
            "<<" => Some(Operation::ShiftLeft),
            ">>" => Some(Operation::ShiftRight),
            "&&" => Some(Operation::BitwiseAnd),
            "||" => Some(Operation::BitwiseOr),
            _ => None,
        }
    }

    fn new(expr: &[String]) -> Self {
        let op = Math::get_op(expr[1].as_str());
        if let Some(op) = op {
            let (lhs, lhs_is_hex) = Math::check_num(expr[0].as_str());
            let (rhs, rhs_is_hex) = Math::check_num(expr[2].as_str());
            return Math {
                lhs: Math::hexstr_to_int(lhs, lhs_is_hex),
                op,
                rhs: Math::hexstr_to_int(rhs, rhs_is_hex),
                res: 0,
            };
        };
        std::process::exit(-1);
    }

    fn add(&mut self) {
        self.res = self.lhs.wrapping_add(self.rhs);
    }

    fn sub(&mut self) {
        self.res = self.lhs.wrapping_sub(self.rhs);
    }

    fn mul(&mut self) {
        self.res = self.lhs.wrapping_mul(self.rhs);
    }

    fn div(&mut self) {
        self.res = self.lhs.wrapping_div(self.rhs);
    }

    fn and(&mut self) {
        self.res = self.lhs & self.rhs;
    }

    fn or(&mut self) {
        self.res = self.lhs | self.rhs;
    }

    fn rshift(&mut self) {
        self.res = self.lhs.wrapping_shr(self.rhs as u32);
    }

    fn lshift(&mut self) {
        self.res = self.lhs.wrapping_shl(self.rhs as u32);
    }

    fn modulo(&mut self) {
        self.res = self.lhs % self.rhs;
    }

    fn math(&mut self) -> usize {
        match self.op {
            Operation::Addition => self.add(),
            Operation::Subtraction => self.sub(),
            Operation::Multiplication => self.mul(),
            Operation::Division => self.div(),
            Operation::Modulo => self.modulo(),
            Operation::ShiftLeft => self.lshift(),
            Operation::ShiftRight => self.rshift(),
            Operation::BitwiseAnd => self.and(),
            Operation::BitwiseOr => self.or(),
        }
        self.print_res();
        self.res
    }

    fn print_res(&self) {
        println!(
            "\t{:<16}\t\t{:<16x}\t\t{:<#b}",
            self.res, self.res, self.res
        );
    }
}

fn usage() {
    eprintln!("Expected exactly 3 arguments: LHS <operator> RHS!");
    std::process::exit(-1);
}

fn is_single_digit(arg: &str) -> bool {
    arg.split(' ').count() == 1
}

fn main() {
    let env_args: Vec<_> = env::args().collect();
    match env_args.len().cmp(&2) {
        Ordering::Less => usage(),
        Ordering::Equal => {
            let mut args = env_args.as_slice()[1..].to_vec();
            if is_single_digit(&args[0]) {
                args.push("+".to_string());
                args.push("0".to_string());
                let _ = Math::new(&args).math();
            } else {
                let args = args[0]
                    .split(' ')
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>();
                let _ = Math::new(&args).math();
            };
        }
        Ordering::Greater => {
            let args: Vec<String> = if env_args.len() != 4 {
                env_args[1]
                    .trim_matches('"')
                    .split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
            } else {
                env_args.as_slice()[1..].to_vec()
            };
            if args.len() != 3 {
                usage();
            } else {
                let _ = Math::new(&args).math();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_quotes() {
        let args = ["22", "+", "1"].map(|x| x.to_string());
        let res = Math::new(&args).math();
        assert_eq!(22 + 1, res);
    }

    #[test]
    fn quotes() {
        let args = "22 * 1"
            .trim_matches('"')
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        let res = Math::new(&args).math();
        assert_eq!(22 * 1, res);
    }
}
