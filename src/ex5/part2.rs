use std::{fs};

#[derive(Clone, Copy)]
struct FreshRange {
    start: u64,
    end: u64,
}

pub fn run() {
    let fresh_list = get_fresh_list();

    let cleaned_list = into_distinc_ranges(&fresh_list);

    let result = cleaned_list
        .into_iter()
        .map(|range| range.end - range.start + 1)
        .reduce(|acc, range| acc + range)
        .unwrap();

    print!("EX 5 PART 1: {}\n", result);
}

fn get_fresh_list() -> Vec<FreshRange> {
    let fresh_list_content = fs::read_to_string("./src/ex5/fresh_list.data")
        .expect("Should have been able to read the file");
    fresh_list_content
        .lines()
        .map(|line| {
            let mut elems = line.split("-");
            FreshRange {
                start: elems.next().unwrap().parse::<u64>().unwrap(),
                end: elems.next().unwrap().parse::<u64>().unwrap(),
            }
        })
        .collect::<Vec<_>>()
}

fn into_distinc_ranges(ranges: &Vec<FreshRange>) -> Vec<FreshRange> {
    let mut result = Vec::<FreshRange>::new();

    for range in ranges {
        let mut cleaned = Vec::<FreshRange>::new();
        cleaned.push(FreshRange {
            start: range.start,
            end: range.end,
        });

        for cleaned_range in &result {
            cleaned = extract_range(&cleaned, cleaned_range);
        }

        for cleaned_range in cleaned {
            result.push(cleaned_range);
        }
    }

    result
}

fn extract_range(ranges: &Vec<FreshRange>, comparison: &FreshRange) -> Vec<FreshRange> {
    let mut result = Vec::<FreshRange>::new();
    for range in ranges {
        if range.end < comparison.start || range.start > comparison.end {
            result.push(FreshRange {
                start: range.start,
                end: range.end,
            });
        } else if range.start < comparison.start && range.end > comparison.end {
            result.push(FreshRange {
                start: range.start,
                end: comparison.start - 1,
            });
            result.push(FreshRange {
                start: comparison.end + 1,
                end: range.end,
            });
        } else if range.start < comparison.start {
            result.push(FreshRange {
                start: range.start,
                end: comparison.start - 1,
            })
        } else if range.end > comparison.end {
            result.push(FreshRange {
                start: comparison.end + 1,
                end: range.end,
            })
        }
    }

    result
}
