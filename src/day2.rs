use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Debug)]
enum Operation {
    Add(usize, usize, usize),
    Multiply(usize, usize, usize),
    Exit,
}

impl From<&[usize]> for Operation {
    fn from(other: &[usize]) -> Self {
        match other[0] {
            1 => Operation::Add(other[1], other[2], other[3]),
            2 => Operation::Multiply(other[1], other[2], other[3]),
            99 => Operation::Exit,
            other => panic!("Unknown opcode {}", other),
        }
    }
}

fn handle_operation(operation: Operation, output: &mut Vec<usize>) -> Option<()> {
    match operation {
        Operation::Add(left, right, index) => {
            output[index] = output[left] + output[right];
            Some(())
        }
        Operation::Multiply(left, right, index) => {
            output[index] = output[left] * output[right];
            Some(())
        }
        Operation::Exit => None,
    }
}

fn run_program(mut program: Vec<usize>) -> Vec<usize> {
    let mut idx = 0;
    loop {
        if program.len() < idx + 4 {
            return program;
        }
        let op = Operation::from(&program[idx..idx + 4]);
        if handle_operation(op, &mut program).is_none() {
            return program;
        }
        idx += 4;
    }
}

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<usize> {
    input.split(',').map(|el| el.parse().unwrap()).collect()
}

#[aoc(day2, part1)]
fn part1(input: &Vec<usize>) -> usize {
    let mut program = input.clone();
    program[1] = 12;
    program[2] = 2;
    run_program(program)[0]
}

const TARGET: usize = 19690720;

#[aoc(day2, part2)]
fn part2(input: &Vec<usize>) -> usize {
    // Optimisation: rather than naively running the program through all inputs,
    // in order, create a vector with all options and binary search using
    // the run_program function.
    // This is only possible because the output is a monotonic function of
    // nouns/verbs (since `slice::binary_search` needs to know where to look
    // for the next test - higher or lower).
    let mut nouns_and_verbs: Vec<(u16, u16)> = Vec::with_capacity(10000);
    for noun in 0..100 {
        for verb in 0..100 {
            nouns_and_verbs.push((noun, verb));
        }
    }
    let mut attempts = 0;
    let res = nouns_and_verbs.binary_search_by(|probe| {
        attempts += 1;
        let mut program = input.clone();
        program[1] = probe.0 as usize;
        program[2] = probe.1 as usize;
        run_program(program)[0].cmp(&TARGET)
    });
    let i = res.expect("No solution found");
    (100 * nouns_and_verbs[i].0 + nouns_and_verbs[i].1) as usize
}

#[cfg(test)]
mod tests {

    use super::run_program;

    #[test]
    fn test_run_program() {
        assert_eq!(run_program(vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);

        assert_eq!(run_program(vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);

        assert_eq!(
            run_program(vec![2, 4, 4, 5, 99, 0]),
            vec![2, 4, 4, 5, 99, 9801]
        );

        assert_eq!(
            run_program(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        )
    }
}
