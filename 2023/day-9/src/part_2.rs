use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space1},
    combinator::{map_res, opt, recognize},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

fn parse_line(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    let (input, numbers) = separated_list1(
        space1,
        map_res(recognize(preceded(opt(tag("-")), digit1)), |n: &str| {
            n.parse::<i32>()
        }),
    )(input)?;

    let mut nums = vec![numbers];

    let mut zeros_found = false;
    let mut round = 0;

    while zeros_found == false {
        let mut next_round = vec![];
        for index in 0..nums[round].len() - 1 {
            next_round.push(nums[round][index + 1] - nums[round][index]);
        }

        nums.push(next_round);
        round += 1;

        if nums[round].iter().filter(|x| x == &&0).count() == nums[round].len() {
            zeros_found = true;
        }
    }

    Ok((input, nums))
}

fn parse_data(input: &str) -> IResult<&str, Vec<Vec<Vec<i32>>>> {
    let (input, numbers) = separated_list1(line_ending, parse_line)(input)?;
    Ok((input, numbers))
}

pub fn process(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let (_, data) = parse_data(input).expect("Expected to parse");

    let answer = data.into_iter().fold(0, |mut total, mut round| {
        round.reverse();
        let firsts: Vec<&i32> = round.iter().map(|x| x.first().unwrap_or(&0)).collect();
        total += firsts.iter().fold(0, |mut c, item| {
            c = **item - c;
            c
        });

        total
    });

    Ok(answer.to_string())
}
