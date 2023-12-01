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
            .split("")
            .map(|col| match col.parse::<i32>() {
                Ok(n) => Some(n),
                _ => None,
            })
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect::<Vec<i32>>();

        let mut str = String::new();

        if let Some(first) = nums.first() {
            str.push_str(&first.to_string());
        }

        if let Some(last) = nums.last() {
            str.push_str(&last.to_string());
        }

        if let Ok(n) = str.parse::<i32>() {
            ans += n;
        }

        ans
    });

    Ok(result.to_string())
}
