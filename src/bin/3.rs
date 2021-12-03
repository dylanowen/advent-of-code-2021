use advent_of_code_2021::example;
use advent_of_code_2021::problem::RunFor;
use advent_of_code_2021::problem::{run, Problem, ProblemState};
use env_logger::Env;

struct Three {}

type Diagnostic = u16;

trait DiagnosticUtil {
    fn get(&self, i: usize) -> bool;
    fn set(&mut self, i: usize);
}

impl DiagnosticUtil for Diagnostic {
    fn get(&self, i: usize) -> bool {
        (self >> i) & 0b1 == 0b1
    }

    fn set(&mut self, i: usize) {
        *self |= 1 << i;
    }
}

impl Problem for Three {
    type Input = (Vec<Diagnostic>, usize);
    type Extra = ();

    fn parse(s: &str, _state: &ProblemState<Self::Extra>) -> Self::Input {
        let diagnostics = s
            .split('\n')
            .map(|line| Diagnostic::from_str_radix(line, 2).unwrap())
            .collect();

        (diagnostics, s.split('\n').next().unwrap().len())
    }

    fn part_1((input, width): &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        let mut counts = [0; 16];
        for diagnostic in input {
            for (i, count) in counts.iter_mut().enumerate().take(*width) {
                if diagnostic.get(i) {
                    *count += 1;
                }
            }
        }

        let mut gamma = 0;
        for (i, count) in counts.iter_mut().enumerate().take(*width) {
            if *count > input.len() / 2 {
                gamma.set(i);
            }
        }
        let epsilon = !gamma & ((1 << width) - 1);

        Some(format!("{}", epsilon as u32 * gamma as u32))
    }

    fn part_2((input, width): &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        let mut oxygen = input.clone();
        let mut co2 = input.clone();
        for i in (0..*width).rev() {
            // for oxygen keep the most common value
            filter_gas(&mut oxygen, i, |zeroes, ones| ones >= zeroes);
            // for c02 keep the least common value
            filter_gas(&mut co2, i, |zeroes, ones| ones < zeroes);
        }
        let oxygen = oxygen[0];
        let co2 = co2[0];

        Some(format!("{}", oxygen as usize * co2 as usize))
    }

    fn problem_number() -> usize {
        3
    }
}

fn filter_gas<F>(gas: &mut Vec<Diagnostic>, index: usize, values_to_keep: F)
where
    F: Fn(usize, usize) -> bool,
{
    if gas.len() > 1 {
        let ones = gas.iter().fold(0, |result, diagnostic| {
            if diagnostic.get(index) {
                result + 1
            } else {
                result
            }
        });
        let zeroes = gas.len() - ones;
        let retain_goal = values_to_keep(zeroes, ones);

        gas.retain(|diagnostic| diagnostic.get(index) == retain_goal);
    }
}

fn main() {
    env_logger::init_from_env(Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn"));

    example!(Three; RunFor::Both, (), r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#);
    run::<Three>((), include_str!("3_input.txt"));
}

#[cfg(test)]
mod one {
    use super::*;
    use advent_of_code_2021::problem::assert_solution;

    #[test]
    fn test() {
        assert_solution::<Three>(include_str!("3_input.txt"), (), "1025636", "793873");
    }
}
