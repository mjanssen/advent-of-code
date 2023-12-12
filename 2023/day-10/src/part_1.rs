type Num = i16;
type Coord = (Num, Num);
type Grid = Vec<Vec<char>>;

#[derive(Debug, Clone, PartialEq)]
struct Node {
    x: Num,
    y: Num,
    char: char,
}

#[derive(Debug, Clone)]
struct Game {
    grid: Grid,
    start: Node,
}

const ADJACENT: [(Num, Num); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

#[derive(Debug)]
enum Origin {
    Top,
    Bottom,
    Left,
    Right,
}

impl Node {
    fn next_postion(&self, previous: &Node, grid: &Grid) -> Option<Node> {
        let entry: Origin = match self {
            n if previous.x < n.x => Origin::Left,
            n if previous.x > n.x => Origin::Right,
            n if previous.y > n.y => Origin::Bottom,
            n if previous.y < n.y => Origin::Top,
            _ => panic!("Shouldn't happen"),
        };

        let coord: Option<Coord> = match self.char {
            'J' => match entry {
                Origin::Left => Some((self.x, self.y - 1)),
                Origin::Top => Some((self.x - 1, self.y)),
                _ => None,
            },
            'F' => match entry {
                Origin::Right => Some((self.x, self.y + 1)),
                Origin::Bottom => Some((self.x + 1, self.y)),
                _ => None,
            },
            '7' => match entry {
                Origin::Left => Some((self.x, self.y + 1)),
                Origin::Bottom => Some((self.x - 1, self.y)),
                _ => None,
            },
            'L' => match entry {
                Origin::Right => Some((self.x, self.y - 1)),
                Origin::Top => Some((self.x + 1, self.y)),
                _ => None,
            },
            '|' => match entry {
                Origin::Bottom => Some((self.x, self.y - 1)),
                Origin::Top => Some((self.x, self.y + 1)),
                _ => None,
            },
            '-' => match entry {
                Origin::Left => Some((self.x + 1, self.y)),
                Origin::Right => Some((self.x - 1, self.y)),
                _ => None,
            },
            _ => None,
        };

        if let Some(c) = coord {
            if c.0 >= 0 && c.0 < grid[0].len() as Num && c.1 >= 0 && c.1 < grid.len() as Num {
                return Some(Node {
                    x: c.0,
                    y: c.1,
                    char: grid[c.1 as usize][c.0 as usize],
                });
            }
        }

        None
    }

    fn get_adjacent<'a>(&'a self, grid: &'a Grid) -> impl Iterator<Item = Node> + '_ {
        ADJACENT
            .iter()
            .filter(|coord| {
                // Check if adjacent squares are within boundaries
                if self.x + (coord.0) < 0 {
                    return false;
                }

                if self.x + (coord.0) > grid[0].len() as Num {
                    return false;
                }

                if self.y + (coord.1) < 0 {
                    return false;
                }

                if self.y + (coord.1) > grid.len() as Num {
                    return false;
                }

                // Filter out '.' chars
                if ['S', '.'].contains(
                    &grid[(self.y + coord.1 as Num) as usize][(self.x + coord.0 as Num) as usize],
                ) {
                    return false;
                }

                true
            })
            .map(|coord| Node {
                x: self.x + coord.0 as Num,
                y: self.y + coord.1 as Num,
                char: grid[(self.y + coord.1 as Num) as usize][(self.x + coord.0 as Num) as usize],
            })
    }
}

impl Game {
    fn default() -> Self {
        Game {
            grid: Vec::new(),
            start: Node {
                x: 0,
                y: 0,
                char: 'S',
            },
        }
    }

    fn generate_grid(&mut self, input: &str) {
        let lines = &input.lines().collect::<Vec<&str>>();
        let mut grid = Vec::new();

        for (index, line) in lines.iter().enumerate() {
            let chars: Vec<char> = line.chars().collect();
            grid.push(chars);

            if line.contains("S") {
                self.start.x = line.find("S").unwrap_or(0) as Num;
                self.start.y = index as Num;
            }
        }

        self.grid = grid;
    }
}

pub fn process(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut game = Game::default();
    game.generate_grid(input);

    let mut steps = 0;

    for node in game.start.get_adjacent(&game.grid) {
        let mut route: Vec<Node> = vec![node.clone()];
        let mut prev: Node = game.start.clone();
        let mut route_steps = 0;

        while let Some(next) = route.pop() {
            if let Some(next_s) = next.next_postion(&prev, &game.grid) {
                route.push(next_s.clone());
                prev = next;

                route_steps += 1;
            }
        }

        steps = std::cmp::max(steps, route_steps);
    }

    Ok(((steps + 1) / 2).to_string())
}
