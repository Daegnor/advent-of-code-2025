use std::{collections::HashMap, fs};

pub fn run() {
    let content = fs::read_to_string("./src/ex7/disposition.data")
        .expect("Should have been able to read the file");

    // Key == column, value == number of worlds where beam is on this col
    let mut beam_cols = HashMap::<usize, u64>::new();

    let lines = content.lines().collect::<Vec<_>>();

    for (i, line) in lines.iter().enumerate() {
        let chars: Vec<char> = line.chars().collect::<Vec<_>>();

        if beam_cols.is_empty() {
            for (i, c) in chars.into_iter().enumerate() {
                if c == 'S' {
                    beam_cols.insert(i, 1);
                }
            }
        } else {
            let mut tmp = HashMap::<usize, u64>::new();

            for beam in beam_cols {
                if chars[beam.0] == '^' {
                    if beam.0 > 0 {
                        *tmp.entry(beam.0 - 1).or_insert(0) += beam.1;
                    }

                    if beam.0 < chars.len() - 1 {
                        *tmp.entry(beam.0 + 1).or_insert(0) += beam.1;
                    }
                } else {
                    *tmp.entry(beam.0).or_insert(0) += beam.1;
                }
            }

            beam_cols = tmp;
        }
    }

    print!(
        "EX 7 PART 2: {}\n",
        beam_cols.into_values().reduce(|a, b| a + b).unwrap()
    );
}
