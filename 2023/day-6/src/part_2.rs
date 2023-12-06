use nom::bytes::complete::is_not;
use nom::character::complete::line_ending;
use nom::sequence::separated_pair;
use nom::{
    character::complete::{digit1, space1},
    multi::separated_list1,
    IResult,
};

fn numbers(input: &str) -> IResult<&str, u64> {
    let (input, _) = is_not("0123456789")(input)?;
    let (input, numbers) = separated_list1(space1, digit1)(input)?;

    Ok((input, numbers.join("").parse::<u64>().unwrap()))
}

fn process_data(input: &str) -> IResult<&str, (u64, u64)> {
    let (input, nums) = separated_pair(numbers, line_ending, numbers)(input)?;

    Ok((input, nums))
}

pub fn process(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let (_, (time, distance)) = process_data(input).expect("Expected parsing");

    let mut multiplier = 0;
    for i in 0..=time {
        if (time - i) * (time - (time - i)) > distance {
            multiplier += 1;
        }
    }

    Ok(multiplier.to_string())
}
