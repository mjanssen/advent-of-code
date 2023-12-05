type Number = u64;

type Min = Number;
type Max = Number;
type DestionationStart = Number;
type Maps = Vec<Vec<((Min, Max), DestionationStart)>>;

#[derive(Debug)]
struct Map {
    seeds: Vec<Number>,
    maps: Maps,
}

impl Map {
    fn default() -> Self {
        Self {
            seeds: vec![],
            maps: vec![vec![]; 7],
        }
    }
}

fn as_number(value: &str) -> Option<Number> {
    match value.parse::<Number>() {
        Ok(n) => Some(n),
        _ => None,
    }
}

fn find_result(n: &Number, maps: &Maps) -> Number {
    let mut find = n.clone();

    for map_type in maps {
        for ((min, max), destination_start) in map_type {
            if &find >= min && &find <= max {
                find = destination_start + (find - min);
                break;
            }
        }
    }

    find
}

pub fn process(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut map: Map = Map::default();
    let mut vector_index: usize = 0;

    for line in input.lines() {
        if line.len() == 0 {
            continue;
        }

        if line.contains("seeds:") {
            map.seeds = line.split_whitespace().filter_map(as_number).collect();
            continue;
        }

        if line.contains("map:") {
            vector_index += 1;
        }

        if line.as_bytes()[0].is_ascii_digit() {
            let numbers: Vec<_> = line.split_whitespace().filter_map(as_number).collect();
            map.maps[vector_index - 1]
                .push(((numbers[1], numbers[1] + numbers[2] - 1), numbers[0]));

            continue;
        }
    }

    let mut ans: Number = std::u64::MAX;

    for seed in map.seeds {
        let res = find_result(&seed, &map.maps);
        ans = std::cmp::min(ans, res);
    }

    Ok(ans.to_string())
}
