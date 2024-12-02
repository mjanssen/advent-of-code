use std::ops::Mul;

pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let (left, right): (Vec<u32>, Vec<u32>) =
        _input.lines().fold((Vec::new(), Vec::new()), |mut acc, x| {
            let mut chars = x.split_whitespace();
            acc.0.push(chars.next().unwrap().parse::<u32>().unwrap());
            acc.1.push(chars.next().unwrap().parse::<u32>().unwrap());

            acc
        });

    let res = left
        .iter()
        .map(|v| v.mul(right.iter().filter(|&rv| rv.eq(v)).count() as u32))
        .sum::<u32>();

    Ok(res.to_string())
}
