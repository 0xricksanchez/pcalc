use std::env;

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
    fn hexstr_to_int(inp: &str) -> usize {
        let tr = if inp.starts_with(HEX_PREFIX) {
            inp.trim_start_matches(HEX_PREFIX)
        } else {
            inp
        };

        let i = usize::from_str_radix(tr.trim(), 16);
        if let Ok(val) = i {
            val
        } else {
            tr.trim().parse::<usize>().unwrap()
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
            return Math {
                lhs: Math::hexstr_to_int(expr[0].as_str()),
                op,
                rhs: Math::hexstr_to_int(expr[2].as_str()),
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

    fn math(&mut self) {
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

fn main() {
    let env_args: Vec<_> = env::args().collect();
    if env_args.len() < 2 {
        usage();
    } else if env_args.len() == 2 {
        let mut args = env_args.as_slice()[1..].to_vec();
        args.push("+".to_string());
        args.push("0".to_string());
        Math::new(&args).math();
    } else {
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
            Math::new(&args).math();
        }
    }
}
