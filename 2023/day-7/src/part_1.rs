use itertools::Itertools;
use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, digit1, line_ending},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
struct Hand {
    score: ScoreTypes,
    card_values: (u8, u8, u8, u8, u8),
    bid: u32,
}

const GAME_ORDER: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum ScoreTypes {
    FiveOfKind = 7,
    FourOfKind = 6,
    FullHouse = 5,
    ThreeOfKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

fn parse_line(input: &str) -> IResult<&str, Hand> {
    use ScoreTypes::*;

    let (input, (card, value)) = separated_pair(
        alphanumeric1,
        tag(" "),
        map_res(digit1, |x: &str| x.parse::<u32>()),
    )(input)?;

    let mut card_occurences = HashMap::new();
    let mut hand_score: ScoreTypes = HighCard;
    let mut card_values: Vec<u8> = vec![];

    for c in card.chars() {
        card_values.push(
            GAME_ORDER.len() as u8 - (GAME_ORDER.iter().position(|&x| x == c).unwrap() as u8) + 1,
        );

        match card_occurences.get(&c) {
            Some(v) => card_occurences.insert(c, v + 1),
            _ => card_occurences.insert(c, 1),
        };
    }

    let value_tuple = card_values
        .into_iter()
        .collect_tuple::<(u8, u8, u8, u8, u8)>()
        .expect("Expected tuple of 5 numbers");

    for (_, occurence) in card_occurences {
        let score = match occurence {
            2 => OnePair,
            3 => ThreeOfKind,
            4 => FourOfKind,
            5 => FiveOfKind,
            _ => HighCard,
        };

        if score == OnePair && hand_score == OnePair {
            hand_score = TwoPair;
        }

        if score == OnePair && hand_score == ThreeOfKind
            || score == ThreeOfKind && hand_score == OnePair
        {
            hand_score = FullHouse;
        }

        if score as u8 > hand_score as u8 {
            hand_score = score;
        }
    }

    Ok((
        input,
        Hand {
            score: hand_score,
            card_values: value_tuple,
            bid: value,
        },
    ))
}

fn parse_data(input: &str) -> IResult<&str, Vec<Hand>> {
    let (input, data) = separated_list1(line_ending, parse_line)(input)?;
    Ok((input, data))
}

pub fn process(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let (_, mut games) = parse_data(input).expect("Expected to parse data");

    games.sort_unstable_by_key(|item| (item.score, item.card_values));

    Ok(games
        .iter()
        .enumerate()
        .fold(0, |mut score, (rank, game)| {
            score += ((rank as u32) + 1) * game.bid;
            score
        })
        .to_string())
}
