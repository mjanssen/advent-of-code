use std::collections::HashMap;

use crate::lib::load_file::load_data_file;

use super::ExecuteResponse;

pub fn execute() -> ExecuteResponse {
    let data = load_data_file("day_10.txt")?;

    let mut cycles = 0;
    let mut x_register = 1;

    let mut crt_cycles = 0;

    let mut signal_strength: HashMap<i32, i32> = HashMap::new();

    println!("Part 2 word");
    data.lines().for_each(|line| {
        let mut x_add = 0;
        let mut cycle_amount = 1;

        match line.split_whitespace().collect::<Vec<&str>>()[..] {
            ["addx", num] => {
                cycle_amount = 2;
                x_add = num.parse::<i32>().unwrap();
            }
            _ => {}
        }

        (0..cycle_amount).for_each(|_| {
            cycles += 1;

            // Part 1
            if cycles == 20 {
                signal_strength.insert(cycles, cycles * x_register);
            } else if (cycles - 20) % 40 == 0 {
                signal_strength.insert(cycles, cycles * x_register);
            }

            // // Part 2 / cycles -1 since the cycles already start counting before, we want to start at 0
            crt_cycles = (cycles - 1) % 40;

            let range = x_register - 1..x_register + 2;

            if range.contains(&crt_cycles) {
                print!("#");
            } else {
                print!(".");
            }

            if crt_cycles == 39 {
                println!(" <- Cycle {}", cycles);
            }
        });

        x_register += x_add;
    });

    let answer: i32 = signal_strength.into_iter().map(|(_cycle, num)| num).sum();

    println!("\npart 1 - {}", answer);

    Ok(())
}
