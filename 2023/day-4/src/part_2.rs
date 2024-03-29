#[derive(Debug, Clone)]
struct Card {
    number: u16,
    matching: u16,
}

impl Card {
    fn new(number: u16, matching: u16) -> Self {
        Self { number, matching }
    }
}

fn get_numbers(part: Option<&str>) -> Vec<u16> {
    part.unwrap_or(&"")
        .trim()
        .split_whitespace()
        .filter_map(|x| match x.parse::<u16>() {
            Ok(n) => Some(n),
            _ => None,
        })
        .collect::<Vec<u16>>()
}

pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut cards: Vec<Card> = vec![];

    for line in _input.lines() {
        let card_number = line
            .split(":")
            .collect::<Vec<&str>>()
            .first()
            .unwrap_or(&"")
            .split_whitespace()
            .nth(1)
            .unwrap_or(&"");

        let numbers = line.split(":").last().unwrap_or("").trim();
        let parts = numbers.split("|").collect::<Vec<&str>>();
        let winning_numbers = get_numbers(parts.first().copied());
        let play_numbers = get_numbers(parts.last().copied());

        let mut matching = 0;
        for num in play_numbers {
            if winning_numbers.contains(&num) {
                matching += 1;
            }
        }

        let card = Card::new(card_number.parse::<u16>().unwrap_or(0), matching);

        cards.push(card);
    }

    let mut initial = cards.clone();

    let mut ans = 0;

    while let Some(card) = initial.pop() {
        if card.matching > 0 {
            for c in &cards[(card.number) as usize..=(card.number - 1 + card.matching) as usize] {
                initial.push(c.clone());
            }
        }

        ans += 1;
    }

    Ok(ans.to_string())
}
