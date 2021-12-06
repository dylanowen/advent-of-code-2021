use advent_of_code_2021::example;
use advent_of_code_2021::problem::RunFor;
use advent_of_code_2021::problem::{run, Problem, ProblemState};
use env_logger::Env;

struct Four {}

#[derive(Debug, Clone)]
struct Board {
    boxes: [[Box; 5]; 5],
}

#[derive(Debug, Clone, Default)]
struct Box {
    number: usize,
    marked: bool,
}

impl Board {
    fn mark_drawn_number(&mut self, number: usize) -> bool {
        let mut found = vec![];
        for (y, row) in self.boxes.iter_mut().enumerate() {
            for (x, current_box) in row.iter_mut().enumerate() {
                if current_box.number == number {
                    current_box.marked = true;
                    found.push((x, y));
                }
            }
        }

        if !found.is_empty() {
            for (x, y) in found {
                let mut x_marked = true;
                let mut y_marked = true;
                for i in 0..5 {
                    y_marked = y_marked && self.boxes[y][i].marked;
                    x_marked = x_marked && self.boxes[i][x].marked;
                }
                if x_marked || y_marked {
                    return true;
                }
            }
        }
        false
    }

    fn unmarked_sum(&self) -> usize {
        self.boxes
            .iter()
            .map(|row| {
                row.iter()
                    .map(|b| if !b.marked { b.number } else { 0 })
                    .sum::<usize>()
            })
            .sum()
    }
}

impl Problem for Four {
    type Input = (Vec<usize>, Vec<Board>);
    type Extra = ();

    fn parse(s: &str, _state: &ProblemState<Self::Extra>) -> Self::Input {
        let mut input = s.split('\n');
        let drawn_numbers = input
            .next()
            .unwrap()
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        let mut boards = vec![];
        while input.next().is_some() {
            let mut boxes: [[Box; 5]; 5] = Default::default();
            for board_box in &mut boxes {
                *board_box = input
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|n| Box {
                        number: n.parse::<usize>().unwrap(),
                        marked: false,
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            }
            boards.push(Board { boxes })
        }

        (drawn_numbers, boards)
    }

    fn part_1((draws, boards): &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        let mut boards: Vec<Board> = boards.clone();
        for &draw in draws {
            for board in &mut boards {
                if board.mark_drawn_number(draw) {
                    return Some(format!("{}", board.unmarked_sum() * draw));
                }
            }
        }

        panic!("We never found a winner, bad input");
    }

    fn part_2((draws, boards): &Self::Input, state: &ProblemState<Self::Extra>) -> Option<String> {
        let mut draws = draws.iter();
        let mut boards: Vec<Board> = boards.clone();

        // find a loser by trimming all the winners
        while boards.len() > 1 {
            let draw = *draws.next().unwrap();
            boards = boards
                .into_iter()
                .filter_map(|mut board| {
                    if !board.mark_drawn_number(draw) {
                        Some(board)
                    } else {
                        // drop winners
                        None
                    }
                })
                .collect();
        }

        // if we have 1 loser, they're now the winner
        Self::part_1(&(draws.copied().collect(), boards), state)
    }

    fn problem_number() -> usize {
        4
    }
}

fn main() {
    env_logger::init_from_env(Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn"));

    example!(Four; RunFor::Both, (), r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#);
    run::<Four>((), include_str!("4_input.txt"));
}

#[cfg(test)]
mod test {
    use super::*;
    use advent_of_code_2021::problem::assert_solution;

    #[test]
    fn test() {
        assert_solution::<Four>(include_str!("4_input.txt"), (), "2496", "25925");
    }
}
