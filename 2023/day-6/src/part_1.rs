use nom::bytes::complete::is_not;
use nom::character::complete::line_ending;
use nom::sequence::separated_pair;
use nom::{
    character::complete::{digit1, space1},
    combinator::map_res,
    multi::separated_list1,
    IResult,
};

type Pair = Vec<u16>;
type Pairs = (Pair, Pair);

fn numbers(input: &str) -> IResult<&str, Vec<u16>> {
    let (input, _) = is_not("0123456789")(input)?;
    let (input, numbers) =
        separated_list1(space1, map_res(digit1, |x: &str| x.parse::<u16>()))(input)?;

    Ok((input, numbers))
}

fn process_data(input: &str) -> IResult<&str, Pairs> {
    let (input, nums) = separated_pair(numbers, line_ending, numbers)(input)?;

    Ok((input, nums))
}

pub fn process(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let (_, (time, distance)) = process_data(input).expect("Expected parsing");

    Ok(time
        .iter()
        .zip(distance)
        .fold(1, |mut value, (t, d)| {
            let mut multiplier = 0;
            for i in 0..=t.to_owned() {
                if (t - i) * (t - (t - i)) > d {
                    multiplier += 1;
                }
            }

            if multiplier > 0 {
                value *= multiplier;
            }

            value
        })
        .to_string())
}
