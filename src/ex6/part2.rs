use std::{collections::HashSet, fs};

#[derive(Clone)]
struct Problem {
    variables: Vec<i64>,
    operator: char,
}

pub fn run() {
    let problems = get_problems(&extract_columns());

    let result = problems
        .into_iter()
        .map(|p| resolve(&p))
        .reduce(|a, b| a + b)
        .unwrap();

    print!("EX 6 PART 2: {}\n", result);
}

fn extract_columns() -> Vec<Vec<Vec<char>>> {
    let content =
        fs::read_to_string("./src/ex6/math.data").expect("Should have been able to read the file");

    let empty_cols = content
        .lines()
        .map(|line| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == ' ')
                .map(|(i, _)| i)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut separators = HashSet::<usize>::new();

    for line in empty_cols.into_iter() {
        if separators.is_empty() {
            for col in line {
                separators.insert(col);
            }
        } else {
            let mut tmp = separators.clone();
            for col in &separators {
                if !line.contains(col) {
                    tmp.remove(col);
                }
            }
            separators = tmp;
        }
    }

    let mut separator_vec = separators.iter().map(|i| *i).collect::<Vec<_>>();
    separator_vec.sort();

    content
        .lines()
        .map(|line| {
            let mut result = separator_vec
                .iter()
                .enumerate()
                .map(|(i, col)| {
                    let chars = line.chars().collect::<Vec<_>>();
                    let start = if i == 0 { 0 } else { separator_vec[i - 1] + 1 };

                    chars[start..*col].iter().copied().collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            let chars = line.chars().collect::<Vec<_>>();
            result.push(
                chars[separator_vec[separator_vec.len() - 1] + 1..]
                    .iter()
                    .copied()
                    .collect::<Vec<_>>(),
            );

            result
        })
        .collect::<Vec<_>>()
}

fn get_problems(columns: &Vec<Vec<Vec<char>>>) -> Vec<Problem> {
    let col_len = columns[0].len();
    let mut problems = Vec::<Problem>::new();

    for prob_index in 0..col_len {
        let mut variables = Vec::<i64>::new();
        let mut current = Vec::<char>::new();

        let prob_columns = columns[0][prob_index].len();
        let mut operator = '+';

        for j in 0..prob_columns {
            for i in 0..columns.len() - 1 {
                if columns[i][prob_index][j] != ' ' {
                    current.push(columns[i][prob_index][j]);
                }
            }

            variables.push(
                current
                    .iter()
                    .copied()
                    .collect::<String>()
                    .parse::<i64>()
                    .unwrap(),
            );
            current.clear();
        }

        for i in 0..prob_columns {
            if columns[columns.len() - 1][prob_index][i] != ' ' {
                operator = columns[columns.len() - 1][prob_index][i];
            }
        }

        problems.push(Problem {
            variables,
            operator: operator,
        })
    }

    problems
}

fn resolve(problem: &Problem) -> i64 {
    problem
        .variables
        .iter()
        .map(|v| *v)
        .reduce(|acc, v| match problem.operator {
            '+' => acc + v,
            '-' => acc - v,
            '*' => acc * v,
            '/' => acc / v,
            _ => 0,
        })
        .unwrap()
}
