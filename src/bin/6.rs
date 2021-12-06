use env_logger::Env;

use std::mem::swap;

use advent_of_code_2021::example;
use advent_of_code_2021::problem::RunFor;
use advent_of_code_2021::problem::{run, Problem, ProblemState};

struct Six {}

impl Problem for Six {
    type Input = Vec<usize>;
    type Extra = ();

    fn parse(s: &str, _state: &ProblemState<Self::Extra>) -> Self::Input {
        s.split(',').map(|n| n.parse::<usize>().unwrap()).collect()
    }

    fn part_1(fishes: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        Some(format!("{}", breed_fish(fishes, 80)))
    }

    fn part_2(fishes: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        Some(format!("{}", breed_fish(fishes, 256)))
    }

    fn problem_number() -> usize {
        5
    }
}

fn breed_fish(fish_days: &[usize], days: usize) -> usize {
    let mut fish_counts = [0; 9];
    for &fish in fish_days {
        fish_counts[fish] += 1;
    }

    let mut previous = 0;
    for _ in 0..days {
        for fish_count in fish_counts.iter_mut().rev() {
            swap(fish_count, &mut previous)
        }
        fish_counts[6] += previous;
        fish_counts[8] += previous;
        previous = 0;
    }

    fish_counts.iter().sum()
}

fn main() {
    env_logger::init_from_env(Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn"));

    example!(Six; RunFor::Both, (), "3,4,3,1,2");
    run::<Six>((), include_str!("6_input.txt"));
}

#[cfg(test)]
mod test {
    use advent_of_code_2021::problem::assert_solution;

    use super::*;

    #[test]
    fn test() {
        assert_solution::<Six>(include_str!("6_input.txt"), (), "362666", "1640526601595");
    }
}
