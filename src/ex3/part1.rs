use std::fs;

pub fn run() {
    let contents = fs::read_to_string("./src/ex3/banks1.data")
        .expect("Should have been able to read the file");

    let banks = contents.lines();

    let mut total_power = 0;

    for bank in banks {
        let voltages = bank
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>();
        let mut tenth = voltages[0];
        let mut index = 0;

        for (i, voltage) in voltages[..(voltages.len() - 1)].iter().enumerate() {
            if *voltage > tenth {
                tenth = *voltage;
                index = i;
            }
        }

        let mut unit = voltages[voltages.len() - 1];

        for voltage in &voltages[(index + 1)..] {
            if *voltage > unit {
                unit = *voltage;
            }
        }

        total_power += tenth * 10 + unit;
    }

    print!("EX 3 PART 1: {}\n", total_power);
}
