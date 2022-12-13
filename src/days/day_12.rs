use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

use crate::lib::load_file::load_data_file;

use super::ExecuteResponse;

static ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn blocks_arounds(&self, board: &Vec<Vec<MapItem>>) -> Vec<Self> {
        let mut result: Vec<Self> = vec![];

        // DIRECTION UP
        if self.y > 0 {
            result.push(Self {
                x: self.x,
                y: self.y - 1,
            });
        }

        // DIRECTION RIGHT
        if self.x < board[0].len() - 1 {
            result.push(Self {
                x: self.x + 1,
                y: self.y,
            });
        }

        // DIRECTION DOWN
        if self.y < board.len() - 1 {
            result.push(Self {
                x: self.x,
                y: self.y + 1,
            });
        }

        // DIRECTION LEFT
        if self.x > 0 {
            result.push(Self {
                x: self.x - 1,
                y: self.y,
            });
        }

        result
    }
}

#[derive(Debug, Clone, Default)]
struct MapItem {
    height: isize,
}

#[derive(Debug, PartialEq, Eq)]
struct DijkstraNode {
    cost: isize,
    position: Position,
}

impl Ord for DijkstraNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for DijkstraNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

static PUZZLE_PART: u8 = 1;

pub fn execute() -> ExecuteResponse {
    let data = load_data_file("day_12.txt")?;

    let (start, finish, board) = build_board(&data);

    let mut steps: HashSet<_> = HashSet::new();
    steps.insert(start);

    let mut dijkstra = BinaryHeap::new();

    if PUZZLE_PART == 2 {
        dijkstra.push(DijkstraNode {
            cost: 0,
            position: finish,
        });
    } else {
        dijkstra.push(DijkstraNode {
            cost: 0,
            position: start,
        });
    }

    while let Some(node) = dijkstra.pop() {
        let current = &board[node.position.y][node.position.x];
        if PUZZLE_PART == 2 && current.height == 0 {
            println!("Part 2 - {:?}", node.cost);
            break;
        } else if PUZZLE_PART != 2 && node.position == finish {
            println!("Part 1 - {:?}", node.cost);
            break;
        }

        let blocks_around: Vec<_> = node.position.blocks_arounds(&board);

        let candidates: Vec<_> = blocks_around
            .iter()
            .filter(|c_pos| {
                // Dereference so we don't have to borrow any numbers during comparing
                let next_block_height = *&board[c_pos.y][c_pos.x].height;
                // We have to step down (switch statement below)
                if PUZZLE_PART == 2 {
                    return next_block_height >= current.height - 1;
                }
                next_block_height <= current.height + 1
            })
            .collect();

        for candidate in candidates {
            if steps.insert(*candidate) {
                dijkstra.push(DijkstraNode {
                    cost: node.cost + 1,
                    position: *candidate,
                })
            }
        }
    }

    Ok(())
}

fn build_board(data: &str) -> (Position, Position, Vec<Vec<MapItem>>) {
    let rows = data.lines().count();
    // Get horizontal length, using next to get first item of lines
    let cols = data.lines().next().unwrap_or("").len();

    let mut map: Vec<Vec<MapItem>> = vec![vec![MapItem::default(); cols]; rows];

    let mut start: Position = Position { x: 0, y: 0 };
    let mut finish: Position = Position { x: 0, y: 0 };

    data.lines().enumerate().for_each(|(vertical_index, item)| {
        item.chars().enumerate().for_each(|(horizontal_index, c)| {
            let height: isize = match c {
                'S' => {
                    start = Position {
                        x: horizontal_index,
                        y: vertical_index,
                    };

                    0
                }
                'E' => {
                    finish = Position {
                        x: horizontal_index,
                        y: vertical_index,
                    };

                    25
                }
                _ => match ALPHABET.find(c) {
                    Some(n) => n as isize,
                    _ => panic!("Wth?"),
                },
            };

            map[vertical_index][horizontal_index] = MapItem { height };
        });
    });

    (start, finish, map)
}
