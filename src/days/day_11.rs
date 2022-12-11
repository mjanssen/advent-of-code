use std::collections::HashMap;

use regex::Regex;

use crate::lib::load_file::load_data_file;

use super::ExecuteResponse;

#[derive(Debug)]
struct Monkey {
    no: usize,
    inspects: i32,
    items: Vec<f64>,
    operation: (String, String, String),
    test_number: i64,
    target_true: usize,
    target_false: usize,
}

impl Monkey {
    fn get_worry_level(&self, item: f64) -> f64 {
        let (one, operator, two) = &self.operation;
        let a: f64 = match one.as_str() {
            "old" => item,
            _ => match one.parse::<f64>() {
                Ok(n) => n,
                _ => item,
            },
        };

        let b: f64 = match two.as_str() {
            "old" => item,
            _ => match two.parse::<f64>() {
                Ok(n) => n,
                _ => item,
            },
        };

        let worry_level = match operator.as_str() {
            "*" => a * b,
            "+" => a + b,
            _ => a * b,
        };

        (worry_level as f64 / 3.0).floor()
    }

    fn get_item(&mut self) -> Option<f64> {
        self.items.pop()
    }

    fn check_if_divisible(&self, worry_level: i64) -> bool {
        (worry_level % self.test_number) == 0
    }

    fn get_target_monkey(&self, worry_level: i64) -> usize {
        if self.check_if_divisible(worry_level) {
            return self.target_true;
        }

        self.target_false
    }
}

pub fn execute() -> ExecuteResponse {
    let data = load_data_file("day_11.txt")?;

    let playable_rounds = 20;

    let mut monkeys = parse_monkeys(&data);

    let mut throwed_items: HashMap<usize, Vec<f64>> = HashMap::new();

    for _ in 0..playable_rounds {
        for monkey in monkeys.iter_mut() {
            // First receive items if needed
            match throwed_items.get_mut(&monkey.no) {
                Some(items) => {
                    for item in items {
                        monkey.items.insert(0, *item);
                    }

                    // Reset throwed items for this monkey
                    throwed_items.insert(monkey.no, vec![]);
                }
                _ => {}
            }

            for _ in 0..monkey.items.len() {
                if let Some(item) = monkey.get_item() {
                    let worry_level = monkey.get_worry_level(item);
                    let target = monkey.get_target_monkey(worry_level as i64);

                    // println!("Monkey {} throws {} to {}", monkey.no, worry_level, target);

                    match throwed_items.get_mut(&target) {
                        Some(items) => items.insert(items.len(), worry_level),
                        _ => {
                            throwed_items.insert(target, vec![worry_level]);
                            ()
                        }
                    }

                    monkey.inspects += 1;
                }
            }
        }
    }

    // Settle all dangling throwed items
    monkeys.iter_mut().for_each(|monkey| {
        match throwed_items.get(&monkey.no) {
            Some(items) => {
                monkey.items.append(&mut items.to_vec());

                // Reset throwed items for this monkey
                throwed_items.insert(monkey.no, vec![]);
            }
            _ => {}
        }
    });

    // Sort monkeys by inspects
    monkeys.sort_by(|a, b| b.inspects.cmp(&a.inspects));
    println!("part 1 - {}", monkeys[0].inspects * monkeys[1].inspects);

    monkeys.iter().for_each(|monkey| {
        println!("Monkey {} did: {} inspects", monkey.no, monkey.inspects);
        println!("Monkey {} has: {:?}", monkey.no, monkey.items);
    });

    Ok(())
}

fn parse_monkeys(data: &str) -> Vec<Monkey> {
    let number_regex = Regex::new(r"\d{1,}").unwrap();
    let operation_regex = Regex::new(r"=\s(.+?)\s(.+?)\s(.+?)$").unwrap();

    data.split("\n\n")
        .enumerate()
        .map(|(monkey_index, data)| {
            let mut items: Vec<f64> = vec![];
            let mut operation: (String, String, String) =
                ("".to_string(), "".to_string(), "".to_string());

            let mut test_number = 0;

            let mut target_true = 0;
            let mut target_false = 0;

            let lines = data.lines().collect::<Vec<&str>>();

            // Collect items for monkey
            for cap in number_regex.captures_iter(lines[1]) {
                match &cap[0].parse::<f64>() {
                    Ok(n) => items.push(*n),
                    _ => (),
                }
            }

            items.reverse();

            // Set operation for monkey
            for cap in operation_regex.captures_iter(lines[2]) {
                operation = (cap[1].to_string(), cap[2].to_string(), cap[3].to_string());
            }

            // Test number
            for cap in number_regex.captures_iter(lines[3]) {
                test_number = match &cap[0].parse::<i64>() {
                    Ok(n) => *n,
                    _ => 0,
                };
            }

            let mut last_two = lines[4].to_owned();
            last_two.push_str(lines[5]);
            let mut i = 0;
            for cap in number_regex.captures_iter(last_two.as_str()) {
                let n = match &cap[0].parse::<usize>() {
                    Ok(n) => *n,
                    _ => 0,
                };

                if i == 0 {
                    target_true = n
                } else {
                    target_false = n
                };

                i += 1;
            }

            Monkey {
                no: monkey_index,
                inspects: 0,
                items,
                operation,
                test_number,
                target_true,
                target_false,
            }
        })
        .collect()
}
