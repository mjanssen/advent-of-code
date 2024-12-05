use std::iter::zip;

pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let (mut left, mut right): (Vec<u32>, Vec<u32>) =
        _input.lines().fold((Vec::new(), Vec::new()), |mut acc, x| {
            let mut chars = x.split_whitespace();
            acc.0.push(chars.next().unwrap().parse::<u32>().unwrap());
            acc.1.push(chars.next().unwrap().parse::<u32>().unwrap());

            acc
        });

    left.sort_unstable();
    right.sort_unstable();

    let res = zip(left, right).map(|(l, r)| l.abs_diff(r)).sum::<u32>();

    Ok(res.to_string())
}
