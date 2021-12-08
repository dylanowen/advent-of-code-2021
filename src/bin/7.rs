use env_logger::Env;

use advent_of_code_2021::example;
use advent_of_code_2021::problem::RunFor;
use advent_of_code_2021::problem::{run, Problem, ProblemState};

struct Seven {}

impl Problem for Seven {
    type Input = Vec<usize>;
    type Extra = ();

    fn parse(s: &str, _state: &ProblemState<Self::Extra>) -> Self::Input {
        s.split(',').map(|n| n.parse::<usize>().unwrap()).collect()
    }

    fn part_1(positions: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        Some(format!(
            "{}",
            find_cheapest_position(positions, linear_fuel_usage)
        ))
    }

    fn part_2(positions: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        Some(format!(
            "{}",
            find_cheapest_position(positions, exponential_fuel_usage)
        ))
    }

    fn problem_number() -> usize {
        5
    }
}

fn find_cheapest_position<F>(positions: &[usize], cost_fn: F) -> usize
where
    F: Fn(&[usize], usize) -> usize,
{
    let &min = positions.iter().min().unwrap();
    let &max = positions.iter().max().unwrap();

    let mut least_expensive = usize::MAX;
    for position in min..=max {
        let cost = cost_fn(positions, position);
        if cost < least_expensive {
            least_expensive = cost;
        }
    }

    least_expensive
}

fn distances(positions: &[usize], position: usize) -> Vec<usize> {
    positions
        .iter()
        .map(|&p| {
            if p > position {
                p - position
            } else {
                position - p
            }
        })
        .collect()
}

fn linear_fuel_usage(positions: &[usize], position: usize) -> usize {
    distances(positions, position).iter().sum()
}

fn exponential_fuel_usage(positions: &[usize], position: usize) -> usize {
    distances(positions, position)
        .iter()
        .map(|&distance| {
            // https://en.wikipedia.org/wiki/Triangular_number
            (distance.pow(2) + distance) / 2
        })
        .sum()
}

fn main() {
    env_logger::init_from_env(Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn"));

    example!(Seven; RunFor::Both, (), "16,1,2,0,4,2,7,1,2,14");
    run::<Seven>((), include_str!("7_input.txt"));
}

#[cfg(test)]
mod test {
    use advent_of_code_2021::problem::assert_solution;

    use super::*;

    #[test]
    fn test() {
        assert_solution::<Seven>(include_str!("7_input.txt"), (), "344535", "95581659");
    }
}
