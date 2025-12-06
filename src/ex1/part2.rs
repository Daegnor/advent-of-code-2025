use std::fs;

const MAX: i32 = 100;

pub fn run() {
    let contents = fs::read_to_string("./src/ex1/rotations2.data")
        .expect("Should have been able to read the file");

	let parts = contents.lines();

	let mut current = 50;
	let mut result = 0;

    for line in parts {
		let next = rotate(current, line);

		result += next.1;

		current = next.0;

		if(current == 0) {
			result += 1;
		}
	}

	print!("EX 1 PART 2: {}\n", result);
}

fn rotate(current: i32, instruction: &str) -> (i32, i32) {
	let direction = instruction.chars().nth(0).unwrap();
	let value = instruction[1..].parse::<f32>().unwrap();
	let maxF = MAX as f32;

	let fullRotations = (value / maxF).floor() as i32;
	let remaining = (value % maxF) as i32;

	let steps = if direction == 'L' { current - remaining } else { current + remaining };
	let next = modulo(steps, MAX);

	let mut pass = (value / maxF).floor() as i32;

	if current > 0 && steps < 0 || steps > MAX {
		pass += 1;
	}
	
	
	print!("CURRENT {} INSTRUCTION {} NEXT {} PASS {}\n", current, instruction, next, pass);

	(modulo(next, MAX), pass)
}

fn modulo(a: i32, b: i32) -> i32 {
	((a % b) + b) % b
}