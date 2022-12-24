use std::{collections::HashSet, ops::Range};

use crate::lib::load_file::load_data_file;

use super::ExecuteResponse;

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct Coord {
    x: isize,
    y: isize,
    z: isize,
}

impl Coord {
    fn get_open_sides(&self, set: &HashSet<Coord>) -> i32 {
        let (x, y, z) = (self.x, self.y, self.z);
        let mut covered_sides = 0;

        // Check horizontally
        if set.contains(&Coord { x: x - 1, y, z }) {
            covered_sides += 1;
        }

        if set.contains(&Coord { x: x + 1, y, z}) {
            covered_sides += 1;
        }

        // Check vertically
        if set.contains(&Coord { x, y: y - 1, z }) {
            covered_sides += 1;
        }

        if set.contains(&Coord { x, y: y + 1, z }) {
            covered_sides += 1;
        }

        // Check Z axis
        if set.contains(&Coord { x, y, z: z - 1 }) {
            covered_sides += 1;
        }

        if set.contains(&Coord { x, y, z: z + 1 }) {
            covered_sides += 1;
        }

        // 6 since there's only 6 sides
        6 - covered_sides
    }
}

fn get_coord_ranges(set: &HashSet<Coord>) -> ((isize, isize), (isize, isize), (isize, isize)) {
    // Build ranges for X
    let mut x: (isize, isize) = (0, 0);
    set.clone().iter().map(|c| c.x).for_each(|v| {
        if v < x.0 || x.0 == 0 { x.0 = v }
        if v > x.1 { x.1 = v }
    });

    let mut y: (isize, isize) = (0, 0);
    set.clone().iter().map(|c| c.y).for_each(|v| {
        if v < y.0 || y.0 == 0 { y.0 = v }
        if v > y.1 { y.1 = v }
    });

    let mut z: (isize, isize) = (0, 0);
    set.clone().iter().map(|c| c.z).for_each(|v| {
        if v < z.0 || z.0 == 0 { z.0 = v }
        if v > z.1 { z.1 = v }
    });

    (x, y, z)
}

pub fn execute() -> ExecuteResponse {
    let data = load_data_file("day_18.txt")?;
    let mut coords: HashSet<Coord> = HashSet::new();

    data.lines()
        .filter(|i| i.is_empty() == false)
        .for_each(|line| {
        let c: Vec<&str> = line.split(",").collect();

        coords.insert(Coord {
            x: c[0].parse::<isize>().unwrap(),
            y: c[1].parse::<isize>().unwrap(),
            z: c[2].parse::<isize>().unwrap(),
        });
    });

    let sum: i32 = coords.iter().map(|coord| {
        coord.get_open_sides(&coords)
    }).sum();

    println!("part 1 - {sum:}");

    let coord_ranges = get_coord_ranges(&coords);

    println!("{:?}", coord_ranges);


    let mut empty_ranges: Vec<Coord> = Vec::new();

    for z in coord_ranges.2.0..=coord_ranges.2.1 {
        for x in coord_ranges.0.0..=coord_ranges.0.1 {

            let mut range: (isize, isize) = (-1, -1);

            for y in coord_ranges.1.0..=coord_ranges.1.1 {
           
                let previous_was_block = match y.cmp(&0) {
                    std::cmp::Ordering::Greater => coords.contains(&Coord {x, y: y - 1, z}),
                    _ => false,
                };

                let current_is_block = coords.contains(&Coord {x, y, z});

                if previous_was_block && current_is_block == false {
                    range.0 = y;
                }

                match current_is_block {
                    true => print!("x"),
                    false => print!("."),
                };

                if previous_was_block == false && current_is_block && range.0 > -1 {
                    range.1 = y;

                    for y in range.0..range.1 {
                        empty_ranges.push(Coord { x, y, z });
                        coords.insert(Coord {x, y, z});
                    }

                    range = (-1, -1);
                } 
            }
            println!("");
        }
        println!("");
    }

    let sum: i32 = coords.iter().map(|coord| {
        coord.get_open_sides(&coords)
    }).sum();

    println!("part 2 - {sum:}");

    Ok(())
}
