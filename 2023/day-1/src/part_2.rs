const NUMS: [(&'static str, &'static str); 9] = [
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
];

pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let result = _input.lines().fold(0, |mut ans, mut _line| {
        let mut line_value = String::from(_line);
        for (num_str, replacement) in NUMS {
            line_value = line_value.clone().replace(
                num_str,
                (num_str.to_string() + replacement + num_str).as_str(),
            );
        }

        let nums = line_value
            .chars()
            .filter_map(|x| x.to_digit(10))
            .collect::<Vec<u32>>();

        ans += (10 * nums.first().unwrap_or(&0)) + nums.last().unwrap_or(&0);

        ans
    });

    Ok(result.to_string())
}
