use std::fs;

const MAX: i32 = 100;

pub fn run() {
    let contents = fs::read_to_string("./src/ex1/rotations1.data")
        .expect("Should have been able to read the file");

	let parts = contents.lines();

	let mut current = 50;
	let mut result = 0;

    for line in parts {
		current = rotate(current, line);
		if current == 0 {
			result += 1;
		}
	}

	print!("EX 1 PART 1: {}\n", result);
}

fn rotate(current: i32, instruction: &str) -> i32 {
	let direction = instruction.chars().nth(0).unwrap();
	let value = &instruction[1..].parse::<i32>().unwrap();

	if direction == 'L' {
		return (((current - value) % MAX) + MAX) % MAX;
	}

	(((current + value) % MAX) + MAX) % MAX
}
