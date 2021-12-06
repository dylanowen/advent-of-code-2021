use advent_of_code_2021::example;
use advent_of_code_2021::problem::RunFor;
use advent_of_code_2021::problem::{run, Problem, ProblemState};
use env_logger::Env;

struct Two {}

enum Direction {
    Forward(usize),
    Down(usize),
    Up(usize),
}

impl Problem for Two {
    type Input = Vec<Direction>;
    type Extra = ();

    fn parse(s: &str, _state: &ProblemState<Self::Extra>) -> Self::Input {
        s.split('\n')
            .map(|line| {
                let mut line = line.split(' ');
                let direction = line.next().unwrap();
                let distance = line.next().unwrap().parse::<usize>().unwrap();
                match direction {
                    "forward" => Direction::Forward(distance),
                    "down" => Direction::Down(distance),
                    "up" => Direction::Up(distance),
                    _ => panic!("Bad input"),
                }
            })
            .collect()
    }

    fn part_1(input: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        let mut x = 0;
        let mut depth = 0;
        for step in input.iter() {
            match step {
                Direction::Forward(distance) => x += *distance,
                Direction::Down(distance) => depth += *distance,
                Direction::Up(distance) => depth -= *distance,
            }
        }

        Some(format!("{}", x * depth))
    }

    fn part_2(input: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        let mut aim = 0;
        let mut x = 0;
        let mut depth = 0;
        for step in input.iter() {
            match step {
                Direction::Forward(distance) => {
                    x += distance;
                    depth += aim * distance;
                }
                Direction::Down(distance) => aim += *distance,
                Direction::Up(distance) => aim -= *distance,
            }
        }

        Some(format!("{}", x * depth))
    }

    fn problem_number() -> usize {
        2
    }
}

fn main() {
    env_logger::init_from_env(Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn"));

    example!(Two; RunFor::Both, (), r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#);
    run::<Two>((), include_str!("2_input.txt"));
}

#[cfg(test)]
mod test {
    use super::*;
    use advent_of_code_2021::problem::assert_solution;

    #[test]
    fn test() {
        assert_solution::<Two>(include_str!("2_input.txt"), (), "1660158", "1604592846");
    }
}
