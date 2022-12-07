use std::fs;

mod tests;
mod types;

fn main() {
    let input = fs::read_to_string("./input/input.txt").expect("could not load input file");

    println!("Step 1 result: {}", process_part1(input.clone()));
    println!("Step 2 result: {}", process_part2(input.clone()));
}


fn process_part1(input: String) -> String {
    let (mut stack, instructions) = parse_data(input);

    // execute instructions on stack
    for command in instructions {
        for _ in 0..command.amount {
            let tmp = stack[command.from-1].pop().unwrap();
            stack[command.to-1].push(tmp);
        }
    }

    get_crates_on_top(stack)
}


fn process_part2(input: String) -> String {
    let (mut stack, instructions) = parse_data(input);

    // execute instructions on stack
    for command in instructions {
        let mut tmp: Vec<char> = Vec::new();
        
        for _ in 0..command.amount {
            tmp.push(stack[command.from-1].pop().unwrap());
        }
        tmp.reverse();
        stack[command.to-1].append(&mut tmp);
    }

    get_crates_on_top(stack)
}


// ---------------------------------------------------------------------------------------------------------------------


fn parse_data(input: String) -> (Vec<types::Stack>, Vec<types::Instruction>) {
    let (stack_str, instructions_str) = input.split_once("\n\n").unwrap();

    // Prepare stack data structure and initialize Vec for each stack
    let stack_by_line = stack_str.lines().collect::<Vec<_>>();
    // stack_by_line.pop();

    let mut stack: Vec<types::Stack> = Vec::new();
    let stack_count = (stack_by_line.get(0).expect("No stack information found").len() / 4) + 1;
    for _ in 0..stack_count { stack.push(Vec::new()); }

    // Parse stack by reversing the line first and skip the (useless) enumeration
    for line in stack_by_line.iter().rev().skip(1) {
        let chars: types::Stack = line.chars().collect();
        for pos in (1..line.len()-1).step_by(4) {
            if chars[pos] != ' ' {
                // --> Stack index = ([Current position in line] - 1) / [length of a column]
                stack[(pos - 1) / 4].push(chars[pos]);
            }
        }
    }

    // Parse instructions
    let mut instructions: Vec<types::Instruction> = vec![];
    for line in instructions_str.lines() {
        let mut parser = line.split_ascii_whitespace().filter_map(|s| s.parse::<usize>().ok() );
        let amount = parser.next().unwrap();
        let from = parser.next().unwrap();
        let to = parser.next().unwrap();

        instructions.push(types::Instruction{ amount, from, to });
    }

    (stack, instructions)
}


fn get_crates_on_top(stacks: Vec<Vec<char>>) -> String {
    let solution = stacks.iter()
        .fold("".to_string(), |prev, current_stack| {
            let last = current_stack.len() - 1;
            format!("{}{}", prev, current_stack[last])
        });

    solution
}
