pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let result = _input.lines().fold(0, |mut ans, _line| {
        let nums = _line
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
