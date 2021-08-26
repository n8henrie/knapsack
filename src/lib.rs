use std::fmt;
use std::str::FromStr;

use pyo3::exceptions::PyException;
use pyo3::prelude::*;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

macro_rules! err {
    ($($tt:tt)*) => {
        Err(Box::<dyn std::error::Error>::from($($tt)*))
    }
}

#[pyfunction]
fn solve(input: &str) -> PyResult<String> {
    let problem: Problem = input.parse().unwrap();
    match problem.solve() {
        Ok(solution) => Ok(solution.to_string()),
        Err(e) => Err(PyException::new_err(e.to_string())),
    }
}

fn permutations<T: Clone>(v: &mut [T]) -> Vec<Vec<T>> {
    fn heaps_alg<T>(v: &mut [T], n: usize) -> Vec<Vec<T>>
    where
        T: Clone,
    {
        if n == 1 {
            return vec![v.to_vec()];
        }

        let mut perms = Vec::new();
        for x in 0..n - 1 {
            perms.extend(heaps_alg(v, n - 1));
            if n % 2 == 0 {
                v.swap(n - 1, x);
            } else {
                v.swap(n - 1, 0);
            }
        }
        perms.extend(heaps_alg(v, n - 1));
        perms
    }
    let len = v.len();
    heaps_alg(v, len)
}

impl Problem {
    fn solve(&self) -> Result<Solution> {
        let mut items = self.items.clone();
        let perms = permutations(&mut items);
        let mut solutions: Vec<_> = perms
            .iter()
            .map(|perm| {
                let mut perm = perm.clone();
                let mut bag = <Vec<Item>>::new();
                loop {
                    if let Some(item) = perm.pop() {
                        if bag.iter().map(|i| i.weight).sum::<u32>() + item.weight <= self.capacity
                        {
                            bag.push(item);
                        }
                    } else {
                        let mut indices = vec![false; self.items.len()];
                        for item in &bag {
                            indices[item.index] = true;
                        }
                        return Solution {
                            value: bag.iter().map(|i| i.value).sum(),
                            is_optimal: true,
                            included: indices,
                        };
                    }
                }
            })
            .collect();
        solutions.sort_by_key(|s| s.value);
        if let Some(sol) = solutions.pop() {
            Ok(sol)
        } else {
            err!("No solutions found")
        }
    }
}

#[derive(Debug)]
struct Solution {
    value: u32,
    is_optimal: bool,
    included: Vec<bool>,
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} {}", self.value, self.is_optimal as u8)?;
        write!(
            f,
            "{}",
            self.included
                .iter()
                .map(|&v| (v as u8).to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

#[derive(Debug, PartialEq)]
struct Problem {
    capacity: u32,
    items: Vec<Item>,
}

#[derive(Clone, Debug, PartialEq)]
struct Item {
    index: usize,
    value: u32,
    weight: u32,
}

impl FromStr for Item {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut split = s.split_whitespace();
        if let (Some(value), Some(weight)) = (split.next(), split.next()) {
            let value = value.parse()?;
            let weight = weight.parse()?;
            Ok(Item {
                value,
                weight,
                index: 0,
            })
        } else {
            err!(format!("Can't create Item from {}", s))
        }
    }
}

impl FromStr for Problem {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut lines = s.lines();
        let first_line = lines.next().ok_or_else(|| Error::from("No first line"))?;
        let mut split = first_line.split_whitespace();
        let (count, capacity): (usize, u32) =
            if let (Some(count), Some(capacity)) = (split.next(), split.next()) {
                Ok((count.parse()?, capacity.parse()?))
            } else {
                err!(format!("Can't parse first line: {}", first_line))
            }?;

        let items: Vec<Item> = lines
            .enumerate()
            .map(|(idx, line)| {
                let mut item: Item = line.parse()?;
                item.index = idx;
                Ok(item)
            })
            .collect::<Result<_>>()?;

        if items.len() != count {
            return err!(format!(
                "Mismatch between item count and collected items: {}, {:?}",
                count, items
            ));
        };

        Ok(Problem { capacity, items })
    }
}

#[pymodule]
fn knapsack(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(solve, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve() -> Result<()> {
        let input = std::fs::read_to_string("data/ks_4_0")?;
        let output = solve(&input)?;
        let expected = "19 1
0 0 1 1";
        assert_eq!(output, expected);
        Ok(())
    }

    #[test]
    fn test_parse() -> Result<()> {
        let input = "4 11
8 4
10 5
15 8
4 3";
        let result: Problem = input.parse()?;
        let expected = Problem {
            capacity: 11,
            items: vec![
                Item {
                    index: 0,
                    value: 8,
                    weight: 4,
                },
                Item {
                    index: 1,
                    value: 10,
                    weight: 5,
                },
                Item {
                    index: 2,
                    value: 15,
                    weight: 8,
                },
                Item {
                    index: 3,
                    value: 4,
                    weight: 3,
                },
            ],
        };
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_display_solution() {
        let solution = Solution {
            value: 19,
            is_optimal: true,
            included: vec![false, false, true, true],
        };
        assert_eq!(
            solution.to_string(),
            "19 1
0 0 1 1"
        );
    }
}
