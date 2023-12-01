pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(_input
        .lines()
        .fold(0, |mut ans, _line| {
            let nums = _line
                .chars()
                .filter_map(|x| x.to_digit(10))
                .collect::<Vec<u32>>();

            ans += (10 * nums.first().unwrap_or(&0)) + nums.last().unwrap_or(&0);

            ans
        })
        .to_string())
}
