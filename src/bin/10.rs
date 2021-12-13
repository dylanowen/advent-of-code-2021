use env_logger::Env;

use advent_of_code_2021::example;
use advent_of_code_2021::problem::RunFor;
use advent_of_code_2021::problem::{run, Problem, ProblemState};

use std::default::Default;

trait Deliminator {
    fn is_open(&self) -> bool;

    fn pair(&self) -> char;
}

impl Deliminator for char {
    fn is_open(&self) -> bool {
        matches!(self, '(' | '[' | '{' | '<')
    }

    fn pair(&self) -> char {
        match self {
            '(' => ')',
            ')' => '(',
            '[' => ']',
            ']' => '[',
            '{' => '}',
            '}' => '{',
            '<' => '>',
            '>' => '<',
            _ => '?',
        }
    }
}

struct Ten {}

impl Problem for Ten {
    type Input = Vec<Result<Vec<char>, char>>;
    type Extra = ();

    fn parse(s: &str, _state: &ProblemState<Self::Extra>) -> Self::Input {
        s.split('\n')
            .map(|line| {
                let mut stack = vec![];
                for c in line.chars() {
                    if c.is_open() {
                        stack.push(c);
                    } else if stack.pop().unwrap().pair() != c {
                        return Err(c);
                    }
                }

                Ok(stack)
            })
            .collect()
    }

    fn part_1(syntax_stacks: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        let error_score = syntax_stacks
            .iter()
            .filter_map(|result| {
                if let Err(found) = result {
                    Some(match found {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => 0,
                    })
                } else {
                    None
                }
            })
            .sum::<usize>();

        Some(format!("{}", error_score))
    }

    fn part_2(syntax_stacks: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        let mut completion_scores = syntax_stacks
            .iter()
            .cloned()
            .filter_map(|result| {
                if let Ok(line) = result {
                    let line_score = line.iter().rev().fold(0u64, |result, c| {
                        let char_score = match c.pair() {
                            ')' => 1,
                            ']' => 2,
                            '}' => 3,
                            '>' => 4,
                            _ => 0,
                        };

                        (result * 5) + char_score
                    });

                    Some(line_score)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        completion_scores.sort_unstable();
        let middle_score = completion_scores[completion_scores.len() / 2];

        Some(format!("{}", middle_score))
    }

    fn problem_number() -> usize {
        10
    }
}

fn main() {
    env_logger::init_from_env(Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn"));

    example!(Ten; RunFor::Both, (), r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#);
    run::<Ten>((), include_str!("10_input.txt"));
}

#[cfg(test)]
mod test {
    use advent_of_code_2021::problem::assert_solution;

    use super::*;

    #[test]
    fn test() {
        assert_solution::<Ten>(include_str!("10_input.txt"), (), "367059", "1952146692");
    }
}
