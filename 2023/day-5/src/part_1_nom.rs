use nom::bytes::complete::tag;
use nom::bytes::complete::{take_till, take_until};
use nom::character::{
    complete::{alpha1, digit1, line_ending, newline, space1},
    is_alphabetic, is_digit,
};
use nom::combinator::map_res;
use nom::multi::{many1, many_till, separated_list1};
use nom::sequence::{pair, preceded, separated_pair, tuple};
use nom::IResult;

use std::ops::Range;

#[derive(Debug)]
enum MapType {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}
type RangeTuple = (Range<u16>, Range<u16>);

#[derive(Debug)]
struct Map {
    map_type: MapType,
    range: RangeTuple,
}

impl Map {
    fn new(map_type: &str, (destination_range, source_range): RangeTuple) -> Self {
        Self {
            map_type: match map_type {
                "seed-to-soil" => MapType::SeedToSoil,
                "soil-to-fertilizer" => MapType::SoilToFertilizer,
                "fertilizer-to-water" => MapType::FertilizerToWater,
                "water-to-light" => MapType::WaterToLight,
                "light-to-temperature" => MapType::LightToTemperature,
                "temperature-to-humidity" => MapType::TemperatureToHumidity,
                "humidity-to-location" => MapType::HumidityToLocation,
                _ => panic!("Type not found"),
            },
            range: (destination_range, source_range),
        }
    }
}

fn parse_numbers(input: &str) -> IResult<&str, u16> {
    let (input, number) = map_res(digit1, |s: &str| s.parse::<u16>())(input)?;
    Ok((input, number))
}

fn parse_name(input: &str) -> IResult<&str, ()> {
    dbg!(input);
    Ok((input, ()))
}

fn line(input: &str) -> IResult<&str, Map> {
    let (_, name_until) = take_until(" map:")(input)?;
    let (_, names) = separated_list1(tag("\n\n"), parse_name)(input)?;
    // let (f, ) = take_till(|x: char| x.eq(&'\n'))(name_until)?;
    dbg!(names);
    let name = "seed-to-soil";
    // let (name, _) = take_until("\n\n")(name_until)?;

    let (input, _) = take_till(|c: char| c.is_digit(10))(input)?;
    let (input, nums) = take_until("\n")(input)?;
    let (_, numbers) = separated_list1(space1, parse_numbers)(nums)?;

    let m = Map::new(
        name.trim(),
        (
            (numbers[0]..numbers[0] + numbers[2]),
            (numbers[1]..numbers[1] + numbers[2]),
        ),
    );

    Ok((input, m))
}

fn parse(input: &str) -> IResult<&str, Vec<Map>> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) = separated_list1(space1, parse_numbers)(input)?;
    let (input, maps) = separated_list1(line_ending, line)(input)?;

    Ok((input, maps))
}

pub fn process(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let (_, maps) = parse(input).expect("Expected parsing");


    Ok("".to_string())
}
