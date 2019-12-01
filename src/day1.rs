use aoc_runner_derive::{aoc, aoc_generator};

fn calculate_fuel(mass: usize) -> usize {
    mass / 3 - 2
}

fn calculate_fuel_part2(mass: usize) -> usize {
    std::iter::successors(Some(mass), |m| (m / 3).checked_sub(2))
        .skip(1)
        .sum()
}

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<usize> {
    input.lines().map(|el| el.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
fn part1(input: &Vec<usize>) -> usize {
    input.iter().map(|i| calculate_fuel(*i)).sum()
}

#[aoc(day1, part2)]
fn part2(input: &Vec<usize>) -> usize {
    input.iter().map(|i| calculate_fuel_part2(*i)).sum()
}

#[cfg(test)]
mod tests {

    use super::{calculate_fuel, calculate_fuel_part2};

    #[test]
    fn test_part1() {
        assert_eq!(calculate_fuel(12), 2);
        assert_eq!(calculate_fuel(14), 2);
        assert_eq!(calculate_fuel(1969), 654);
        assert_eq!(calculate_fuel(100756), 33583);
    }

    #[test]
    fn test_part2() {
        assert_eq!(calculate_fuel_part2(12), 2);
        assert_eq!(calculate_fuel_part2(14), 2);
        assert_eq!(calculate_fuel_part2(1969), 966);
        assert_eq!(calculate_fuel_part2(100756), 50346);
    }
}
