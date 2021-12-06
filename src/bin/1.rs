use advent_of_code_2021::problem::{run, Problem, ProblemState};
use env_logger::Env;

struct One {}

impl Problem for One {
    type Input = Vec<usize>;
    type Extra = ();

    fn parse(s: &str, _state: &ProblemState<Self::Extra>) -> Self::Input {
        s.split('\n')
            .map(|depth| depth.parse::<usize>().expect("parse error"))
            .collect()
    }

    fn part_1(depths: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        let mut increases = 0;
        let mut last = depths[0];
        for &depth in &depths[1..] {
            if depth > last {
                increases += 1;
            }
            last = depth;
        }

        Some(format!("{}", increases))
    }

    fn part_2(depths: &Self::Input, state: &ProblemState<Self::Extra>) -> Option<String> {
        let windows = depths
            .windows(3)
            .map(|window| window.iter().sum())
            .collect();

        Self::part_1(&windows, state)
    }

    fn problem_number() -> usize {
        1
    }
}

fn main() {
    env_logger::init_from_env(Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn"));

    run::<One>((), include_str!("1_input.txt"));
}

#[cfg(test)]
mod test {
    use super::*;
    use advent_of_code_2021::problem::assert_solution;

    #[test]
    fn test() {
        assert_solution::<One>(include_str!("1_input.txt"), (), "1583", "1627");
    }
}
