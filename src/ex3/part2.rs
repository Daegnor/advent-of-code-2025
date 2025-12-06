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

        let mut powered = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let mut start = 0;

        for i in 0..12 {
            let mut start_tmp = start;
            powered[i] = voltages[start];

            print!("I {} START {}\n", i, start);

            for (index, voltage) in voltages[start..(voltages.len() - 12 + i + 1)]
                .into_iter()
                .enumerate()
            {
                if *voltage > powered[i] {
                    powered[i] = *voltage;
                    start_tmp = index + start;
                }
            }
            start = start_tmp + 1;
        }

        print!("POWERED {:?}\n", powered);

        total_power += powered
            .iter()
            .enumerate()
            .map(|(index, voltage)| (*voltage as u64) * 10_u64.pow(12 - (index as u32) - 1))
            .reduce(|a, b| a + b)
            .unwrap();
    }

    print!("EX 3 PART 2: {}\n", total_power);
}
