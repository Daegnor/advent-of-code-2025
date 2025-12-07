use std::fs;

struct FreshRange {
    start: u64,
    end: u64,
}

pub fn run() {
    let ingredients = get_ingredients();
    let fresh_list = get_fresh_list();

    let mut fresh = 0;

    for ingredient in ingredients {
        if fresh_list
            .iter()
            .any(|range| range.start <= ingredient && range.end >= ingredient)
        {
            fresh += 1;
        }
    }

    print!("EX 5 PART 1: {}\n", fresh);
}

fn get_ingredients() -> Vec<u64> {
    let ingredients_content = fs::read_to_string("./src/ex5/ingredients.data")
        .expect("Should have been able to read the file");

    ingredients_content
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>()
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
