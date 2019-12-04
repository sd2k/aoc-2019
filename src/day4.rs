use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
fn parse_input(input: &str) -> (usize, usize) {
    let mut iter = input.split('-').map(|el| el.parse().unwrap());
    (iter.next().unwrap(), iter.next().unwrap())
}

fn n_adjacent(password: &str, n: usize) -> usize {
    password
        .as_bytes()
        .windows(n)
        .map(|window| window.iter().all(|el| el == &window[0]) as usize)
        .sum::<usize>()
}

fn has_at_least_two_adjacent(password: &str) -> bool {
    n_adjacent(password, 2) >= 1
}

fn has_at_least_one_exact_pair(password: &str) -> bool {
    n_adjacent(password, 2) > n_adjacent(password, 3)
}

fn never_decrease(password: &str) -> bool {
    password
        .as_bytes()
        .iter()
        .try_fold(
            &0u8,
            |acc, digit| {
                if digit >= acc {
                    Ok(digit)
                } else {
                    Err(())
                }
            },
        )
        .is_ok()
}

fn is_valid_part1(password: &str) -> bool {
    has_at_least_two_adjacent(password) && never_decrease(password)
}

fn is_valid_part2(password: &str) -> bool {
    has_at_least_one_exact_pair(password) && never_decrease(password)
}

#[aoc(day4, part1)]
fn part1(input: &(usize, usize)) -> u16 {
    ((input.0)..=(input.1))
        .map(|pw| pw.to_string())
        .filter_map(|pw| if never_decrease(&pw) { Some(pw) } else { None })
        .map(|pw| has_at_least_two_adjacent(&pw) as u16)
        .sum()
}

#[aoc(day4, part2)]
fn part2(input: &(usize, usize)) -> u16 {
    ((input.0)..=(input.1))
        .map(|pw| pw.to_string())
        .filter_map(|pw| if never_decrease(&pw) { Some(pw) } else { None })
        .map(|pw| has_at_least_one_exact_pair(&pw) as u16)
        .sum()
}

#[cfg(test)]
mod tests {

    use super::{is_valid_part1, is_valid_part2};

    #[test]
    fn test_is_valid_part1() {
        assert_eq!(is_valid_part1("111111"), true);
        assert_eq!(is_valid_part1("223450"), false);
        assert_eq!(is_valid_part1("123789"), false);
    }

    #[test]
    fn test_is_valid_part2() {
        assert_eq!(is_valid_part2("112233"), true);
        assert_eq!(is_valid_part2("123444"), false);
        assert_eq!(is_valid_part2("111122"), true);
    }
}
