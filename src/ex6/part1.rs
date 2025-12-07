use std::fs;

#[derive(Clone)]
struct Problem {
    variables: Vec<i64>,
    operator: char,
}

pub fn run() {
    let problems = get_problems();

    let result = problems
        .into_iter()
        .map(|p| resolve(&p))
        .reduce(|a, b| a + b)
        .unwrap();

    print!("EX 6 PART 1: {}\n", result);
}

fn get_problems() -> Vec<Problem> {
    let content =
        fs::read_to_string("./src/ex6/math.data").expect("Should have been able to read the file");

    let tmp = content
        .lines()
        .map(|line| {
            line.split(' ')
                .filter(|element| element.trim().len() > 0)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let columns = tmp[0].len();

    let mut problems = Vec::<Problem>::new();
    for _ in 0..columns {
        problems.push(Problem {
            variables: Vec::new(),
            operator: '+',
        });
    }

    for (index, line) in tmp.iter().enumerate() {
        if index == tmp.len() - 1 {
            for (i, c) in line.iter().enumerate() {
                problems[i].operator = c.chars().nth(0).unwrap();
            }
        } else {
            for (i, number) in line.iter().enumerate() {
                problems[i].variables.push(number.parse::<i64>().unwrap());
            }
        }
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
