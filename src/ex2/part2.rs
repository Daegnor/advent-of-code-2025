use std::fs;

pub fn run() {
    let contents =
        fs::read_to_string("./src/ex2/ids2.data").expect("Should have been able to read the file");

    let ranges = contents.lines().flat_map(|line| {
        line.split(",").filter(|s| s.len() > 0).map(|s| {
            let mut split = s.split("-");
            std::ops::Range {
                start: split.nth(0).unwrap().to_owned().parse::<u64>().unwrap(),
                end: split.nth(0).unwrap().to_owned().parse::<u64>().unwrap() + 1,
            }
        })
    });

    let mut badIds = 0;

    for range in ranges {
        for id in range {
            if !is_valid(&id.to_string()) {
                badIds += id;
            }
        }
    }

    print!("EX 2 PART 2: {}\n", badIds);
}

fn is_valid(id: &str) -> bool {
    if id.len() < 2 {
        return true;
    }

    let sizes = std::ops::Range {
        start: 1,
        end: (((id.len() as f32) / 2.0).ceil() + 1.0) as usize,
    };

    for i in sizes {
        let pattern = &id[..i];
        let parts = split_size(id, i);

        if parts.into_iter().all(|part| *part.as_str() == *pattern) {
            return false;
        }
    }

    true
}

fn split_size(id: &str, size: usize) -> Vec<String> {
    let chars: Vec<char> = id.chars().collect();

    chars
        .chunks(size)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>()
}
