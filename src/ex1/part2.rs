use std::fs;

const MAX: f32 = 100.0;

pub fn run() {
    let contents = fs::read_to_string("./src/ex1/rotations2.data")
        .expect("Should have been able to read the file");

	let parts = contents.lines();

	let mut current = 50.0;
	let mut result = 0.0;

    for line in parts {
		let next = rotate(current, line);

		result += next.1;

		current = next.0;

		// Ends on 0
		if current == 0.0 {
			result += 1.0;
		}
	}

	print!("EX 1 PART 2: {}\n", result);
}

fn rotate(current: f32, instruction: &str) -> (f32, f32) {
	let direction = instruction.chars().nth(0).unwrap();
	let value = instruction[1..].parse::<f32>().unwrap();

	// Next value without adaptation to lock
	let next_raw = if direction == 'L' { current - (value % MAX) } else { current + (value % MAX) };

	// Adapt to lock
	let next = modulo(next_raw, MAX);

	// Full rotations
	let mut pass = (value / MAX).floor();

	// Pass by 0
	if current > 0.0 && next_raw < 0.0 || next_raw > MAX {
		pass += 1.0;
	}

	(next, pass)
}

fn modulo(a: f32, b: f32) -> f32 {
	((a % b) + b) % b
}