use std::str::Chars;

// TODO: use FromStr trait
// https://rust-lang-nursery.github.io/rust-cookbook/text/string_parsing.html
// even better, read this: https://adriann.github.io/rust_parser.html

pub enum Operation {
    Addition,
    Multiplication,
}

pub enum Value {
    Number(i64),
    Parentheses(Expression),
}

impl Value {
    fn eval(&self) -> i64 {
        match self {
            Self::Number(n) => *n,
            Self::Parentheses(expr) => expr.eval(),
        }
    }
}

pub struct Instruction {
    op: Operation,
    v: Value,
}

impl Instruction {
    fn parse(input: &mut Chars) -> Instruction {
        println!("parsing instruction from: {:?}", input.as_str());

        trim_spaces(input);

        println!("char: {:?}", input.peekable().peek().unwrap());

        match input.next().unwrap() {
            '+' => Instruction {
                op: Operation::Addition,
                v: Value::Number(parse_number(input)),
            },

            '*' => Instruction {
                op: Operation::Multiplication,
                v: Value::Number(parse_number(input)),
            },

            '(' => {
                let instr = Instruction {
                    op: Operation::Addition,
                    v: Value::Parentheses(Expression::parse(input)),
                };

                let c = input.next().unwrap();
                if c != ')' {
                    panic!();
                }

                instr
            }

            unexpected => panic!("unexpected char: {:?}", unexpected),
        }
    }

    fn apply(&self, input: &mut i64) {
        let v = self.v.eval();

        match self.op {
            Operation::Addition => *input += v,
            Operation::Multiplication => *input *= v,
        }
    }
}

pub struct Expression {
    instructions: Vec<Instruction>,
}

impl Expression {
    pub fn parse(input: &mut Chars) -> Expression {
        println!("parsing line: {:?}", input.as_str());

        let mut expr = Expression {
            instructions: Vec::new(),
        };

        while input.peekable().peek().is_some() {
            trim_spaces(input);
            expr.instructions.push(Instruction::parse(input));
        }

        expr
    }

    pub fn eval(&self) -> i64 {
        let mut v = 0;

        for instr in &self.instructions {
            instr.apply(&mut v)
        }

        v
    }
}

fn trim_spaces(input: &mut Chars) {
    let peeked = *input.by_ref().peekable().peek().unwrap();
    println!("peeked : {:?}, {}", peeked, peeked == ' ');

    while *input.by_ref().peekable().peek().unwrap() == ' ' {
        input.next();
    }
}

fn parse_number(input: &mut Chars) -> i64 {
    let mut str = String::new();
    while input.by_ref().peekable().peek().unwrap().is_numeric() {
        str.push(input.next().unwrap());
    }
    str.parse().unwrap()
}
