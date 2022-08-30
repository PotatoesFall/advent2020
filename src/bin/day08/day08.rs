use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Instruction {
    value: i64,
    typ: InstructionType,
}

#[derive(Debug, Clone)]
enum InstructionType {
    Nop,
    Acc,
    Jmp,
}

fn main() {
    let input = advent2020::read_input("input08.txt");

    let instructions = parse_instructions(input);

    // dbg!(&instructions);

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
    let mut used2 = HashSet::new();

    for (i, instruction) in instructions.iter().enumerate() {
        let pos = i as i64;
        match instruction.typ {
            InstructionType::Acc => {}
            InstructionType::Nop => {
                let could_jump_to = pos + instruction.value;
                if !used.contains(&could_jump_to) && !used2.contains(&could_jump_to) {
                    maybe_nop_to_jmp.push(i);
                    used2.insert(i as i64);
                }
            }
            InstructionType::Jmp => {
                let could_nop_to = pos + 1;
                if !used.contains(&could_nop_to) && !used2.contains(&could_nop_to) {
                    maybe_jmp_to_nop.push(i);
                }
            }
        }
    }

    // dbg!(&maybe_nop_to_jmp);
    // dbg!(&maybe_jmp_to_nop);

    for i in maybe_nop_to_jmp {
        let mut instructions = instructions.clone();
        instructions[i].typ = InstructionType::Jmp;
        if finishes(&instructions) {
            break;
        }
    }

    for i in maybe_jmp_to_nop {
        let mut instructions = instructions.clone();
        instructions[i].typ = InstructionType::Nop;
        if finishes(&instructions) {
            break;
        }
    }
}

fn finishes(instructions: &Vec<Instruction>) -> bool {
    let mut pos: i64 = 0;
    let mut acc: i64 = 0;
    let mut used = HashSet::new();
    loop {
        let instruction;
        match instructions.get(pos as usize) {
            Some(i) => instruction = i,
            None => {
                println!("Part 2 - {}", acc);
                return true;
            }
        }
        used.insert(pos);
        execute(&mut pos, &mut acc, &instruction);
        if used.contains(&pos) {
            return false;
        }
    }
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
