use nom::bytes::complete::{is_not, tag};
use nom::{character::complete::digit0, combinator::map_res, multi::separated_list1, IResult};
use regex::Regex;

fn numbers(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = is_not("0123456789")(input)?;
    let (input, numbers) =
        separated_list1(tag(","), map_res(digit0, |x: &str| x.parse::<u32>()))(input)?;

    Ok((input, numbers))
}

pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let dodont = Regex::new(r"(don't\(\)|do\(\)|mul\((\d+),(\d+)\))").unwrap();

    let res = dodont
        .find_iter(_input)
        .map(|x| x.as_str())
        .fold((true, 0), |(mut enabled, mut total), str| {
            match str {
                "don't()" => enabled = false,
                "do()" => enabled = true,
                _ => {
                    if enabled {
                        if let Ok((_, nums)) = numbers(str) {
                            total += nums[0] * nums[1];
                        }
                    }
                }
            };

            (enabled, total)
        })
        .1;

    Ok(res.to_string())
}
