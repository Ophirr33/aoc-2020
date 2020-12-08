use std::mem::swap;

fn main() -> () {
    let mut boot_code = BootCode::parse(INPUT);
    if let ExecuteResult::Looped {
        acc,
        looping_instruction,
        executed,
    } = boot_code.execute_until_loop_or_end()
    {
        println!(
            "part 1: {} (first loop found at {})",
            acc, looping_instruction
        );
        for i in executed
            .into_iter()
            .enumerate()
            .filter(|(_, b)| *b)
            .map(|(i, _)| i)
        {
            let mut nop = Instruction::NOP;
            swap(&mut boot_code.instructions[i], &mut nop);
            if let ExecuteResult::RanToEnd { acc } = boot_code.execute_until_loop_or_end() {
                println!("part 2: {} (loop was fixed at {})", acc, i);
                break;
            } else {
                swap(&mut boot_code.instructions[i], &mut nop);
            }
        }
    }
}

#[derive(Debug)]
struct BootCode {
    instructions: Vec<Instruction>,
}

#[derive(Debug)]
enum Instruction {
    NOP,
    ACC(isize),
    JMP(isize),
}

#[derive(Debug)]
enum ExecuteResult {
    Looped {
        acc: isize,
        looping_instruction: usize,
        executed: Vec<bool>,
    },
    RanToEnd {
        acc: isize,
    },
}

impl BootCode {
    fn parse(s: &str) -> Self {
        let instructions = s
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(|s| match &s[0..=2] {
                "nop" => Instruction::NOP,
                "acc" => Instruction::ACC(s[4..].parse::<isize>().unwrap()),
                "jmp" => Instruction::JMP(s[4..].parse::<isize>().unwrap()),
                _ => panic!("unknown instruction: {}"),
            })
            .collect::<Vec<_>>();
        BootCode { instructions }
    }

    fn execute_until_loop_or_end(&self) -> ExecuteResult {
        let mut idx: usize = 0;
        let mut prior_idx: usize = 0;
        let mut acc: isize = 0;
        let num_instructions = self.instructions.len();
        let mut executed = vec![false; self.instructions.len()];
        while idx < num_instructions && !executed[idx] {
            executed[idx] = true;
            prior_idx = idx;
            match self.instructions[idx] {
                Instruction::NOP => idx += 1,
                Instruction::ACC(a) => {
                    acc += a;
                    idx += 1;
                }
                Instruction::JMP(j) => idx = (idx as isize + j) as usize,
            }
        }
        if idx >= num_instructions {
            ExecuteResult::RanToEnd { acc }
        } else {
            ExecuteResult::Looped {
                acc,
                executed,
                looping_instruction: prior_idx,
            }
        }
    }
}

const INPUT: &'static str = include_str!("day8.txt");
