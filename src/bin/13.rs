use env_logger::Env;

use advent_of_code_2021::example;
use advent_of_code_2021::problem::RunFor;
use advent_of_code_2021::problem::{run, Problem, ProblemState};

use advent_of_code_2021::coordinates::Grid;
use lazy_static::lazy_static;
use regex::Regex;
use std::default::Default;

struct Thirteen {}

pub enum Fold {
    X(isize),
    Y(isize),
}

impl Problem for Thirteen {
    type Input = (Grid<bool>, Vec<Fold>);
    type Extra = ();

    fn parse(s: &str, _state: &ProblemState<Self::Extra>) -> Self::Input {
        lazy_static! {
            static ref DOT_RE: Regex = Regex::new(r"(\d+),(\d+)").unwrap();
            static ref FOLD_RE: Regex = Regex::new(r"fold along ([xy])=(\d+)").unwrap();
        }

        let mut paper = Grid::new_from_range(0..10, 0..10);
        let mut lines = s.split('\n');
        for line in &mut lines {
            if !line.is_empty() {
                let parsed_dot = DOT_RE.captures(line).unwrap();
                let x = parsed_dot[1].parse::<isize>().unwrap();
                let y = parsed_dot[2].parse::<isize>().unwrap();
                paper.set(x, y, true);
            } else {
                break;
            }
        }

        let mut folds = vec![];
        for line in lines {
            let parsed_fold = FOLD_RE.captures(line).unwrap();
            let index = parsed_fold[2].parse::<isize>().unwrap();
            let fold = match &parsed_fold[1] {
                "x" => Fold::X(index),
                "y" => Fold::Y(index),
                _ => panic!("Unexpected dimension: {}", &parsed_fold[1]),
            };
            folds.push(fold);
        }

        (paper, folds)
    }

    fn part_1((paper, folds): &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        let folded = fold_paper(paper, &folds[0]);

        let dot_count = folded.indices().filter(|&p| *folded.get_point(p)).count();

        Some(format!("{}", dot_count))
    }

    fn part_2((paper, folds): &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        let mut folded = paper.clone();
        for fold in folds {
            folded = fold_paper(&folded, fold);
        }

        let mut result = String::new();
        for y in folded.y_min()..folded.y_max() {
            for x in folded.x_min()..folded.x_max() {
                result.push_str(&format!(
                    "{}",
                    if *folded.get(x as isize, y as isize) {
                        '#'
                    } else {
                        '.'
                    }
                ));
            }
            result.push('\n');
        }

        Some(result)
    }

    fn problem_number() -> usize {
        13
    }
}

fn fold_paper(paper: &Grid<bool>, fold: &Fold) -> Grid<bool> {
    match fold {
        Fold::X(index) => {
            let mut result = Grid::new_from_range(paper.x_min()..*index, paper.y_range());
            for y in paper.y_range() {
                for x in paper.x_min()..*index {
                    let other_x = paper.x_max() - x - 1;
                    result.set(x, y, *paper.get(x, y) || *paper.get(other_x, y));
                }
            }
            result
        }
        Fold::Y(index) => {
            let mut result = Grid::new_from_range(paper.x_range(), paper.y_min()..*index);
            for y in paper.y_min()..*index {
                let other_y = paper.y_max() - y - 1;
                for x in paper.x_range() {
                    result.set(x, y, *paper.get(x, y) || *paper.get(x, other_y));
                }
            }
            result
        }
    }
}

fn main() {
    env_logger::init_from_env(Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn"));

    example!(Thirteen; RunFor::Both, (), r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"#);
    run::<Thirteen>((), include_str!("13_input.txt"));
}

#[cfg(test)]
mod test {
    use advent_of_code_2021::problem::assert_solution;

    use super::*;

    #[test]
    fn test() {
        assert_solution::<Thirteen>(
            include_str!("13_input.txt"),
            (),
            "724",
            r#".##..###....##.###..####.###..#..#.#....
#..#.#..#....#.#..#.#....#..#.#..#.#....
#....#..#....#.###..###..#..#.#..#.#....
#....###.....#.#..#.#....###..#..#.#....
#..#.#....#..#.#..#.#....#.#..#..#.#....
.##..#.....##..###..####.#..#..##..####.
"#,
        );
    }
}
