fn get_numbers(part: Option<&str>) -> Vec<&str> {
    part.unwrap_or(&"")
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>()
}

pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(_input
        .lines()
        .fold(0, |mut ans, line| {
            let numbers = line.split(":").last().unwrap_or("").trim();
            let parts = numbers.split("|").collect::<Vec<&str>>();
            let winning_numbers = get_numbers(parts.first().copied());
            let play_numbers = get_numbers(parts.last().copied());

            let mut multiplied = 0;
            for num in play_numbers {
                if winning_numbers.contains(&num) {
                    if multiplied.eq(&0) {
                        multiplied = 1;
                    } else {
                        multiplied *= 2;
                    }
                }
            }

            ans += multiplied;

            ans
        })
        .to_string())
}
