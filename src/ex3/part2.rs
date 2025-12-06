use std::fs;

pub fn run() {
    let contents = fs::read_to_string("./src/ex3/banks2.data")
        .expect("Should have been able to read the file");

    let banks = contents.lines();

    let mut total_power = 0_u64;

    for bank in banks {
        let voltages = bank
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>();

        let mut power = 0;
        let mut start = 0;

        for i in 0..12 {
            // Get max voltage in allowed range
            let (_, voltage) = voltages[start..voltages.len() - 12 + i + 1]
                .into_iter()
                .enumerate()
                .max_by_key(|(_, voltage)| **voltage)
                .unwrap();

            // Voltage into power (depending on position)
            power += (*voltage as u64) * 10_u64.pow(12 - (i as u32) - 1);

            // max_by_key returns last element, so find first element with value == voltage, add it to start for position in full voltages array, and add 1 for next iteration
            start = voltages[start..]
                .iter()
                .position(|v| *v == *voltage)
                .unwrap()
                + start
                + 1;
        }

        total_power += power;
    }

    print!("EX 3 PART 2: {}\n", total_power);
}
