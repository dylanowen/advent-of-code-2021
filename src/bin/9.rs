use env_logger::Env;
use std::collections::HashSet;

use advent_of_code_2021::coordinates::two_d::{Point, PointLike};
use advent_of_code_2021::coordinates::Grid;
use advent_of_code_2021::example;
use advent_of_code_2021::problem::RunFor;
use advent_of_code_2021::problem::{run, Problem, ProblemState};

use std::default::Default;

struct Nine {}

impl Problem for Nine {
    type Input = Grid<usize>;
    type Extra = ();

    fn parse(s: &str, _state: &ProblemState<Self::Extra>) -> Self::Input {
        let mut height_map = Grid::new_from_range(0..5, 0..5);
        height_map.default = usize::MAX;

        s.split('\n').enumerate().for_each(|(y, line)| {
            for (x, c) in line.chars().enumerate() {
                height_map.set(x as isize, y as isize, c.to_digit(10).unwrap() as usize);
            }
        });

        height_map
    }

    fn part_1(height_map: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        let low_sum = low_points(height_map)
            .iter()
            .map(|&low_point| height_map.get_point(low_point) + 1)
            .sum::<usize>();

        Some(format!("{}", low_sum))
    }

    fn part_2(height_map: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        let mut basin_sizes = low_points(height_map)
            .iter()
            .map(|&low_point| basin_size(HashSet::from([low_point]), HashSet::new(), height_map))
            .collect::<Vec<_>>();
        basin_sizes.sort_unstable();

        let basin_sum = basin_sizes.into_iter().rev().take(3).product::<usize>();

        Some(format!("{}", basin_sum))
    }

    fn problem_number() -> usize {
        9
    }
}

fn low_points(height_map: &Grid<usize>) -> Vec<Point> {
    let mut low_points = vec![];
    for point in height_map.indices() {
        let height = height_map.get_point(point);
        if point
            .neighbors()
            .iter()
            .all(|&n| height < height_map.get_point(n))
        {
            low_points.push(point);
        }
    }

    low_points
}

fn basin_size(
    mut search_points: HashSet<Point>,
    mut found: HashSet<Point>,
    height_map: &Grid<usize>,
) -> usize {
    if !search_points.is_empty() {
        let next = *search_points.iter().next().unwrap();
        search_points.remove(&next);

        if found.insert(next) {
            // we haven't looked at this point yet so insert all the valid neighbors
            for neighbor in next.neighbors() {
                if *height_map.get_point(neighbor) < 9 {
                    search_points.insert(neighbor);
                }
            }
        }

        basin_size(search_points, found, height_map)
    } else {
        found.len()
    }
}

fn main() {
    env_logger::init_from_env(Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn"));

    example!(Nine; RunFor::Both, (), r#"2199943210
3987894921
9856789892
8767896789
9899965678"#);
    run::<Nine>((), include_str!("9_input.txt"));
}

#[cfg(test)]
mod test {
    use advent_of_code_2021::problem::assert_solution;

    use super::*;

    #[test]
    fn test() {
        assert_solution::<Nine>(include_str!("9_input.txt"), (), "548", "786048");
    }
}
