use std::{cmp::min, fs};

pub fn run() {
    let contents =
        fs::read_to_string("./src/ex4/rolls.data").expect("Should have been able to read the file");

    let cases = contents
        .lines()
        .filter(|line| line.len() > 0)
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let columns = cases[0].len();

    let mut result = 0;

    print!("LEN {} {}\n", columns, cases.len());

    for i in 0..cases.len() {
        for j in 0..columns {
            if cases[i][j] == '@' && check_around(&cases, j, i) {
                result += 1;
            }
        }
    }

    print!("EX 4 PART 1: {}\n", result);
}

fn check_around(cases: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let mut count_busy_spaces = 0;

    let min_i = if y > 0 { y - 1 } else { y };
    let min_j = if x > 0 { x - 1 } else { x };

    for i in min_i..min(y + 2, cases.len()) {
        for j in min_j..min(cases[i].len(), x + 2) {
            if !(i == y && j == x) && cases[i][j] == '@' {
                count_busy_spaces += 1;
            }
        }
    }

    count_busy_spaces < 4
}
