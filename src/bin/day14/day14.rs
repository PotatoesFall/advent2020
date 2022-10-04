use core::panic;
use std::collections::HashMap;
use std::num::ParseIntError;

struct Program {
    onemask: u64,
    zeromask: u64,
    x_indices: Vec<usize>,

    // both parts
    instructions: Vec<Instruction>,
}

#[derive(Clone, Copy)]
struct Instruction {
    dst: u64,
    v: u64,
}

fn main() {
    let input = advent2020::read_input("input14.txt");
    // let input = advent2020::read_input("input14-example.txt");

    let programs: Vec<Program> = input
        .split("mask = ")
        .skip(1)
        .map(|x| parse_program(x.trim().to_owned()).unwrap())
        .collect();

    println!("Part 1 - {}", part1(&programs));
    println!("Part 2 - {}", part2(&programs));
}

fn part1(programs: &Vec<Program>) -> u64 {
    let mut memory = HashMap::new();
    for program in programs {
        for instruction in &program.instructions {
            let mut v = instruction.v;
            v |= program.onemask;
            v &= !program.zeromask;

            memory.insert(instruction.dst, v);
        }
    }

    memory.values().sum()
}

fn part2(programs: &Vec<Program>) -> u64 {
    let mut memory = HashMap::new();
    for program in programs {
        for instruction in &program.instructions {
            for addr in get_addresses(instruction.dst | program.onemask, &program.x_indices) {
                memory.insert(addr, instruction.v);
            }
        }
    }

    memory.values().sum()
}

fn get_addresses(addr: u64, x_indices: &Vec<usize>) -> Vec<u64> {
    let mut addrs = vec![addr];

    for x_index in x_indices {
        let mask = 1 << x_index;
        for i in 0..addrs.len() {
            if addrs[i] & mask == mask {
                addrs.push(addrs[i] & !(mask));
            } else {
                addrs.push(addrs[i] | mask);
            }
        }
    }

    addrs
}

fn parse_program(str: String) -> Result<Program, ParseIntError> {
    let mut onemask = 0;
    let mut zeromask = 0;
    let mut x_indices = Vec::new();
    let chars: Vec<char> = str.chars().collect();
    for i in 0..36 {
        match chars[i] {
            'X' => x_indices.push(35 - i),
            '1' => {
                onemask = onemask + (1 << (35 - i));
            }
            '0' => {
                zeromask = zeromask + (1 << (35 - i));
            }
            _ => panic!("{}", chars[i]),
        }
    }
    // println!("{:#038b} {:#038b} {:?}", onemask, zeromask, &str[0..36]);

    let mut instructions = Vec::new();

    for line in str.split("\n").skip(1) {
        let instruction = parse_instruction(line)?;
        instructions.push(instruction);
    }

    Ok(Program {
        onemask,
        zeromask,
        x_indices,
        instructions,
    })
}

fn parse_instruction(line: &str) -> Result<Instruction, ParseIntError> {
    let mut end = 0;
    for (i, _) in line.char_indices() {
        if line.bytes().nth(i).unwrap() == ']' as u8 {
            end = i;
            break;
        }
    }
    if end == 0 {
        panic!("] not found in instruction: {}", line);
    }

    Ok(Instruction {
        dst: (&line[4..end]).parse()?,
        v: (&line[end + 4..]).parse()?,
    })
}
