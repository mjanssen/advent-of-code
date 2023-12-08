use std::collections::HashMap;

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, line_ending},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

type Step = String;

#[derive(Debug)]
pub struct Node {
    current: Step,
    left: Step,
    right: Step,
}

type Steps = HashMap<Step, (Step, Step)>;
type Directions<'a> = Vec<&'a str>;

pub fn process_step(input: &str) -> IResult<&str, Node> {
    let (input, (current, options)) = separated_pair(alpha1, tag(" = "), take_until(")"))(input)?;
    let (_, (left, right)) =
        preceded(tag("("), separated_pair(alpha1, tag(", "), alpha1))(options)?;

    let (input, _) = preceded(tag(")"), tag(""))(input)?;

    Ok((
        input,
        Node {
            current: current.to_string(),
            left: left.to_string(),
            right: right.to_string(),
        },
    ))
}

pub fn process_directions(input: &str) -> IResult<&str, Directions> {
    let (input, directions) = take_until("\n")(input)?;
    let directions = directions.split("").filter(|x| x != &"").collect();

    Ok((input, directions))
}

pub fn process_data(input: &str) -> IResult<&str, (Directions, Steps)> {
    let (input, directions) = process_directions(input)?;
    let (input, nodes) = preceded(tag("\n\n"), separated_list1(line_ending, process_step))(input)?;

    let mut steps: HashMap<_, _> = HashMap::new();
    for node in nodes {
        steps.insert(node.current, (node.left, node.right));
    }

    Ok((input, (directions, steps)))
}

pub fn process(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let (_input, (directions, steps)) = process_data(input).expect("Expected to parse data");

    let mut step_amount = 0u32;
    let mut step_index = 0u32;
    let mut current: Step = "AAA".to_string();

    while current.ne(&"ZZZ") {
        if step_index == directions.len() as u32 {
            step_index = 0;
        }
        
        current = match directions[step_index as usize] {
            "L" => steps.get(&current).unwrap().0.clone(),
            "R" => steps.get(&current).unwrap().1.clone(),
            _ => panic!("Shouldn't happen")
        };

        step_amount += 1;
        step_index += 1;
    }
    
    Ok(step_amount.to_string())
}
