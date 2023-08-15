use super::scanner::Scanner;

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
    fn parse(input: &mut Scanner) -> Instruction {
        trim_spaces(input);

        // return early if just a number
        if input.peek().unwrap().is_numeric() {
            return Instruction {
                op: Operation::Addition,
                v: Value::Number(parse_number(input)),
            };
        }

        match input.pop().unwrap() {
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

                let c = input.pop().unwrap();
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
    pub fn parse(input: &mut Scanner) -> Expression {
        let mut expr = Expression {
            instructions: Vec::new(),
        };

        while input.peek().is_some() {
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

fn trim_spaces(input: &mut Scanner) {
    let peeked = input.peek();
    if peeked.is_none() {
        return;
    }

    while input.peek().eq(&Option::from(' ')) {
        input.pop();
    }
}

fn parse_number(input: &mut Scanner) -> i64 {
    trim_spaces(input);

    println!("about to parse_number: {:?}", input.str());

    let mut str = String::new();
    while input.peek().is_some() && input.peek().unwrap().is_numeric() {
        str.push(input.pop().unwrap());
    }
    str.parse().unwrap()
}
