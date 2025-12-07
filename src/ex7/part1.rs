use std::{collections::HashSet, fs};

pub fn run() {
    let content = fs::read_to_string("./src/ex7/disposition.data")
        .expect("Should have been able to read the file");

    let mut beam_cols = HashSet::<usize>::new();
    let mut split = 0;

    let lines = content.lines().collect::<Vec<_>>();

    for line in lines {
        let chars: Vec<char> = line.chars().collect::<Vec<_>>();

        if beam_cols.is_empty() {
            for (i, c) in chars.into_iter().enumerate() {
                if c == 'S' {
                    beam_cols.insert(i);
                }
            }
        } else {
            let mut tmp = HashSet::<usize>::new();

            for beam in beam_cols {
                if chars[beam] == '^' {
                    if beam > 0 {
                        tmp.insert(beam - 1);
                    }

                    if beam < chars.len() - 1 {
                        tmp.insert(beam + 1);
                    }

                    split += 1;
                } else {
                    tmp.insert(beam);
                }
            }

            beam_cols = tmp;
        }
    }

    print!("EX 7 PART 1: {}\n", split);
}
