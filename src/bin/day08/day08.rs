use std::collections::HashSet;
use std::{fmt, iter};

struct Instruction {
    value: i64,
    typ: InstructionType,
}

enum InstructionType {
    Nop,
    Acc,
    Jmp,
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.typ {
            InstructionType::Nop => f.write_str("Nop"),
            InstructionType::Acc => f.debug_tuple("Acc").field(&self.value).finish(),
            InstructionType::Jmp => f.debug_tuple("Jmp").field(&self.value).finish(),
        }
    }
}

fn main() {
    let input = advent2020::read_input("input/input08.txt");

    let instructions = parse_instructions(input);

    // for instruction in instructions {
    //     match instruction {
    //         Instruction::Nop => println!("Nop"),
    //         Instruction::Acc(v) => println!("Acc {}", v),
    //         Instruction::Jmp(v) => println!("Jmp {}", v),
    //     }
    // }

    let mut pos: i64 = 0;
    let mut acc: i64 = 0;
    let mut used = HashSet::new();
    loop {
        let instruction = instructions.get(pos as usize).unwrap();
        used.insert(pos);
        execute(&mut pos, &mut acc, &instruction);
        if used.contains(&pos) {
            break;
        }
    }

    println!("Part 1 - {}", acc);

    let mut maybe_nop_to_jmp = Vec::new();
    let mut maybe_jmp_to_nop = Vec::new();

    for (i, instruction) in instructions.iter().enumerate() {
        let pos = i as i64;
        match instruction.typ {
            InstructionType::Acc => {}
            InstructionType::Nop => {
                let could_jump_to = pos + instruction.value;
                if !used.contains(&could_jump_to) {
                    maybe_nop_to_jmp.push(i);
                }
            }
            InstructionType::Jmp => {
                let could_nop_to = pos + 1;
                if !used.contains(&could_nop_to) {
                    maybe_jmp_to_nop.push(i);
                }
            }
        }
    }

    dbg!(maybe_nop_to_jmp);
    dbg!(maybe_jmp_to_nop);

    for i in &maybe_nop_to_jmp {
        // I fucking hate this language wtf is going on
        let mut instruction= instructions.get(i.clone()).unwrap();
        let mut owned_instruction:Instruction = instruction.clone().to_owned();
        instruction.typ = InstructionType::Jmp;
        instructions.splice(i..&(i+1), iter::once(instruction));
    }
}

fn finishes(instructions: &Vec<Instruction>) -> bool {
    false
}

fn execute(pos: &mut i64, acc: &mut i64, instruction: &Instruction) {
    // dbg!(&pos, &acc, instruction);
    match instruction.typ {
        InstructionType::Nop => *pos += 1,
        InstructionType::Acc => {
            *acc += instruction.value;
            *pos += 1
        }
        InstructionType::Jmp => *pos += instruction.value,
    }
}

fn parse_instructions(input: String) -> Vec<Instruction> {
    let mut vec = Vec::new();

    for line in input.split("\n") {
        let v = line[4..].parse::<i64>().unwrap();
        if line.strip_prefix("acc").is_some() {
            vec.push(Instruction {
                value: v,
                typ: InstructionType::Acc,
            })
        } else if line.strip_prefix("jmp").is_some() {
            vec.push(Instruction {
                value: v,
                typ: InstructionType::Jmp,
            });
        } else {
            vec.push(Instruction {
                value: v,
                typ: InstructionType::Nop,
            });
        }
    }

    vec
}
