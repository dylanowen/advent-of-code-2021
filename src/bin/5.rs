use env_logger::Env;
use lazy_static::lazy_static;
use regex::Regex;

use advent_of_code_2021::example;
use advent_of_code_2021::problem::RunFor;
use advent_of_code_2021::problem::{run, Problem, ProblemState};

use advent_of_code_2021::two_d::{Grid, PUsize, PointLike};

struct Five {}

impl Problem for Five {
    type Input = Vec<(PUsize, PUsize)>;
    type Extra = ();

    fn parse(s: &str, _state: &ProblemState<Self::Extra>) -> Self::Input {
        lazy_static! {
            static ref VENT_RE: Regex = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
        }

        s.split('\n')
            .map(|line| {
                let parsed_row = VENT_RE.captures(line).unwrap();
                let x1 = parsed_row[1].parse::<usize>().unwrap();
                let y1 = parsed_row[2].parse::<usize>().unwrap();
                let x2 = parsed_row[3].parse::<usize>().unwrap();
                let y2 = parsed_row[4].parse::<usize>().unwrap();

                ((x1, y1), (x2, y2))
            })
            .collect()
    }

    fn part_1(vents: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        Some(format!("{}", calculate_vent_danger(vents, false)))
    }

    fn part_2(vents: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        Some(format!("{}", calculate_vent_danger(vents, true)))
    }

    fn problem_number() -> usize {
        5
    }
}

fn calculate_vent_danger(vents: &[(PUsize, PUsize)], handle_diagonals: bool) -> usize {
    let mut ocean_floor: Grid<usize> = Grid::new(0..10, 0..10, 0);
    for (start, end) in vents {
        if !handle_diagonals && start.x() != end.x() && start.y() != end.y() {
            continue;
        }

        let mut inc = start.distance(*end);
        if inc.x() != 0 {
            *inc.x_mut() /= inc.x();
        }
        if inc.y() != 0 {
            *inc.y_mut() /= inc.y();
        }
        let mut point = *start;
        while &point != end {
            ocean_floor.set(point, ocean_floor.get(point) + 1);
            point = point.add(inc);
        }
        ocean_floor.set(point, ocean_floor.get(point) + 1);
    }

    ocean_floor.enumerate().fold(
        0,
        |result, (_, &danger_level)| {
            if danger_level > 1 {
                result + 1
            } else {
                result
            }
        },
    )
}

fn main() {
    env_logger::init_from_env(Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn"));

    example!(Five; RunFor::Both, (), r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#);
    run::<Five>((), include_str!("5_input.txt"));
}

#[cfg(test)]
mod test {
    use advent_of_code_2021::problem::assert_solution;

    use super::*;

    #[test]
    fn test() {
        assert_solution::<Five>(include_str!("5_input.txt"), (), "8350", "19374");
    }
}
