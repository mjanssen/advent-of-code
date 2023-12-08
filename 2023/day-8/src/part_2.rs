use std::collections::HashMap;

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{alphanumeric1, line_ending},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

type Step<'a> = &'a str;

#[derive(Debug, Clone, PartialEq, Copy)]
enum End {
    A,
    Z,
    X,
}

#[derive(Debug, Clone, Copy)]
pub struct Node<'a> {
    current: Step<'a>,
    left: Step<'a>,
    right: Step<'a>,
    ends_with: End,
}

type Steps<'a> = HashMap<Step<'a>, Node<'a>>;
type Directions<'a> = Vec<&'a str>;

fn process_step(input: &str) -> IResult<&str, Node> {
    let (input, (current, options)) =
        separated_pair(alphanumeric1, tag(" = "), take_until(")"))(input)?;
    let (_, (left, right)) = preceded(
        tag("("),
        separated_pair(alphanumeric1, tag(", "), alphanumeric1),
    )(options)?;

    let (input, _) = preceded(tag(")"), tag(""))(input)?;

    let ends_with = match current.chars().last().unwrap() {
        'A' => End::A,
        'Z' => End::Z,
        _ => End::X,
    };

    Ok((
        input,
        Node {
            current,
            ends_with,
            left,
            right,
        },
    ))
}

fn process_directions(input: &str) -> IResult<&str, Directions> {
    let (input, directions) = take_until("\n")(input)?;
    let directions = directions.split("").filter(|x| x != &"").collect();

    Ok((input, directions))
}

fn process_data(input: &str) -> IResult<&str, (Directions, Steps, Vec<Node>)> {
    let (input, directions) = process_directions(input)?;
    let (input, nodes) = preceded(tag("\n\n"), separated_list1(line_ending, process_step))(input)?;

    let mut steps: HashMap<_, _> = HashMap::new();
    for node in nodes.clone() {
        steps.insert(node.current, node);
    }

    Ok((input, (directions, steps, nodes)))
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }

    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

pub fn process(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let (_input, (directions, steps, nodes)) = process_data(input).expect("Expected to parse data");

    let items: Vec<&Node> = nodes
        .iter()
        .filter(|node| node.ends_with == End::A)
        .collect();

    let results = items
        .into_iter()
        .map(|node| {
            let mut current_node = *node;

            directions
                .iter()
                .cycle()
                .enumerate()
                .find_map(|(index, direction)| {
                    let next = match direction {
                        &"L" => steps.get(&current_node.left).unwrap().clone(),
                        &"R" => steps.get(&current_node.right).unwrap().clone(),
                        _ => panic!("Shouldn't happen"),
                    };

                    if next.ends_with == End::Z {
                        return Some(index + 1);
                    } else {
                        current_node = next;
                        return None;
                    }
                })
                .expect("Expected number")
        })
        .collect::<Vec<usize>>();

    Ok(lcm(&results).to_string())
}
