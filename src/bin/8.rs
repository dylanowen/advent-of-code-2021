use env_logger::Env;
use std::collections::HashSet;

use advent_of_code_2021::example;
use advent_of_code_2021::problem::RunFor;
use advent_of_code_2021::problem::{run, Problem, ProblemState};
use lazy_static::lazy_static;
use regex::Regex;
use std::default::Default;
use std::fmt::{Debug, Formatter};
use std::ops::{Index, IndexMut};
use std::str::from_utf8;

const A: usize = 0;
const B: usize = 1;
const C: usize = 2;
const D: usize = 3;
const E: usize = 4;
const F: usize = 5;
const G: usize = 6;

struct Eight {}

struct Display {
    signals: [DisplayDigit; 10],
    digits: [DisplayDigit; 4],
}

impl Display {
    fn display(&self, transform: &[usize; 7]) -> String {
        self.digits
            .iter()
            .map(|digit| digit.as_digit(transform))
            .collect()
    }

    fn calculate_transform(&self) -> [usize; 7] {
        let mut transform = [0; 7];

        // handle the easy cases first
        let one = self
            .signals
            .iter()
            .find(|d| d.count_segments() == 2)
            .unwrap();
        let four = self
            .signals
            .iter()
            .find(|d| d.count_segments() == 4)
            .unwrap();
        let seven = self
            .signals
            .iter()
            .find(|d| d.count_segments() == 3)
            .unwrap();

        let counts: [usize; 7] = self.signals.iter().fold([0; 7], |mut result, digit| {
            for i in A..=G {
                if digit[i] {
                    result[i] += 1;
                }
            }
            result
        });

        transform[A] = *seven
            .segment_set()
            .difference(&one.segment_set())
            .next()
            .unwrap();
        transform[B] = counts
            .iter()
            .enumerate()
            .find_map(|(i, &c)| if c == 6 { Some(i) } else { None })
            .unwrap();
        transform[C] = counts
            .iter()
            .enumerate()
            .find_map(|(i, &c)| {
                if c == 8 && i != transform[A] {
                    Some(i)
                } else {
                    None
                }
            })
            .unwrap();
        transform[E] = counts
            .iter()
            .enumerate()
            .find_map(|(i, &c)| if c == 4 { Some(i) } else { None })
            .unwrap();
        transform[F] = counts
            .iter()
            .enumerate()
            .find_map(|(i, &c)| if c == 9 { Some(i) } else { None })
            .unwrap();

        // find D by getting 4 and removing all the known pieces
        let mut four_signals = four.segment_set();
        four_signals.remove(&transform[B]);
        four_signals.remove(&transform[C]);
        four_signals.remove(&transform[F]);
        transform[D] = four_signals.into_iter().next().unwrap();

        // find G because it's count should be 7 and it can't be D
        transform[G] = counts
            .iter()
            .enumerate()
            .find_map(|(i, &c)| {
                if c == 7 && i != transform[D] {
                    Some(i)
                } else {
                    None
                }
            })
            .unwrap();

        transform
    }
}

#[derive(Default, Clone)]
struct DisplayDigit {
    segments: [bool; 7],
}

impl Debug for DisplayDigit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let debug_str = self
            .segments
            .iter()
            .enumerate()
            .filter(|(_, &segment)| segment)
            .map(|(i, _)| i as u8 + b'a')
            .collect::<Vec<_>>();
        write!(f, "{}", from_utf8(&debug_str).unwrap())
    }
}

impl DisplayDigit {
    fn count_segments(&self) -> usize {
        self.segments.iter().filter(|&&s| s).count()
    }

    fn segment_set(&self) -> HashSet<usize> {
        HashSet::from_iter(
            self.segments
                .iter()
                .enumerate()
                .filter(|(_, &segment)| segment)
                .map(|(i, _)| i),
        )
    }

    fn as_digit(&self, transform: &[usize; 7]) -> char {
        let mut result = 0u8;
        for (i, &t) in transform.iter().rev().enumerate() {
            result |= (self.segments[t] as u8) << i;
        }

        match result {
            0b1110111 => '0',
            0b0010010 => '1',
            0b1011101 => '2',
            0b1011011 => '3',
            0b0111010 => '4',
            0b1101011 => '5',
            0b1101111 => '6',
            0b1010010 => '7',
            0b1111111 => '8',
            0b1111011 => '9',
            _ => result as char,
        }
    }
}

impl Index<usize> for DisplayDigit {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        &self.segments[index]
    }
}

impl IndexMut<usize> for DisplayDigit {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.segments[index]
    }
}

impl Problem for Eight {
    type Input = Vec<Display>;
    type Extra = ();

    fn parse(s: &str, _state: &ProblemState<Self::Extra>) -> Self::Input {
        lazy_static! {
            static ref DISPLAY_RE: Regex = Regex::new(r"(\w+) (\w+) (\w+) (\w+) (\w+) (\w+) (\w+) (\w+) (\w+) (\w+) \| (\w+) (\w+) (\w+) (\w+)").unwrap();
        }

        s.split('\n')
            .map(|line| {
                let parsed_row = DISPLAY_RE.captures(line).unwrap();
                let mut signals: [DisplayDigit; 10] = Default::default();
                for i in 0..10 {
                    for &signal in parsed_row[i + 1].as_bytes() {
                        signals[i][(signal - b'a') as usize] = true;
                    }
                }

                let mut digits: [DisplayDigit; 4] = Default::default();
                for i in 0..4 {
                    for &signal in parsed_row[i + 11].as_bytes() {
                        digits[i][(signal - b'a') as usize] = true;
                    }
                }

                Display { signals, digits }
            })
            .collect()
    }

    fn part_1(displays: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        let simple_count = displays
            .iter()
            .map(|display| {
                let out = display
                    .digits
                    .iter()
                    .filter(|d| {
                        let segment_count = d.count_segments();

                        segment_count == 2
                            || segment_count == 4
                            || segment_count == 3
                            || segment_count == 7
                    })
                    .count();

                out
            })
            .sum::<usize>();

        Some(format!("{}", simple_count))
    }

    fn part_2(displays: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        let result = displays
            .iter()
            .map(|display| {
                let transform = display.calculate_transform();
                let result = display.display(&transform);

                result
                    .parse::<usize>()
                    .unwrap_or_else(|_| panic!("Got a non number: {}", result))
            })
            .sum::<usize>();

        Some(format!("{}", result))
    }

    fn problem_number() -> usize {
        8
    }
}

fn main() {
    env_logger::init_from_env(Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn"));

    example!(Eight; RunFor::Part2, (), r#"acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"#);
    example!(Eight; RunFor::Both, (), r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#);
    run::<Eight>((), include_str!("8_input.txt"));
}

#[cfg(test)]
mod test {
    use advent_of_code_2021::problem::assert_solution;

    use super::*;

    #[test]
    fn test() {
        assert_solution::<Eight>(include_str!("8_input.txt"), (), "365", "975706");
    }
}
