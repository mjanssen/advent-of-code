use regex::Regex;

pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let rx = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let res: u32 = rx
        .captures_iter(_input)
        .map(|c| &c[1].parse::<u32>().unwrap() * &c[2].parse::<u32>().unwrap())
        .sum();

    Ok(res.to_string())
}
