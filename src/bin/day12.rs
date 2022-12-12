use std::collections::{BinaryHeap, HashMap};

use aoc22::input::read_input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input("day12")?;
    let (start, end, heightmap) = parse_input(input);

    let shortest_path_steps = dijkstra(start, end, &heightmap);
    println!("shortest path to end is {} steps", shortest_path_steps);

    let mut lowest_elevations = Vec::new();
    for row in 0..heightmap.len() {
        for col in 0..heightmap[0].len() {
            if heightmap[row][col] == 'a' as u32 {
                lowest_elevations.push((row, col));
            }
        }
    }

    let shortest_path_from_any_low = lowest_elevations
        .iter()
        .map(|s| dijkstra(*s, end, &heightmap))
        .min()
        .unwrap();
    println!(
        "shortest path to end from any low elevation is {} steps",
        shortest_path_from_any_low
    );

    Ok(())
}

type Heightmap = Vec<Vec<u32>>;
type Pos = (usize, usize);

#[derive(Eq, PartialEq)]
struct State {
    pos: Pos,
    cost: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

fn dijkstra(start: Pos, end: Pos, heightmap: &Heightmap) -> u32 {
    let mut dist: HashMap<Pos, u32> = HashMap::new();
    dist.insert(start, 0);

    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    for row in 0..heightmap.len() {
        for col in 0..heightmap[0].len() {
            let pos = (row, col);
            if pos != start {
                dist.insert(pos, u32::MAX - 1);
            }
        }
    }
    heap.push(State {
        pos: start,
        cost: 0,
    });

    while let Some(next) = heap.pop() {
        if next.pos == end {
            return next.cost;
        }

        adjacent(next.pos, &heightmap).iter().for_each(|p| {
            let mut alt = *dist.get(&next.pos).unwrap() + 1;

            if heightmap[p.0][p.1] > heightmap[next.pos.0][next.pos.1]
                && heightmap[p.0][p.1] - heightmap[next.pos.0][next.pos.1] > 1
            {
                alt = u32::MAX;
            }

            if alt < *dist.get(&p).unwrap() {
                dist.insert(*p, alt);
                heap.push(State { pos: *p, cost: alt });
            }
        });
    }
    return *dist.get(&end).unwrap();
}

fn adjacent(pos: Pos, heightmap: &Heightmap) -> Vec<Pos> {
    let mut positions = vec![];
    if pos.0 > 0 {
        positions.push((pos.0 - 1, pos.1));
    }
    if pos.0 < heightmap.len() - 1 {
        positions.push((pos.0 + 1, pos.1));
    }
    if pos.1 > 0 {
        positions.push((pos.0, pos.1 - 1));
    }
    if pos.1 < heightmap[0].len() - 1 {
        positions.push((pos.0, pos.1 + 1));
    }
    positions
}

fn parse_input(input: String) -> (Pos, Pos, Heightmap) {
    let chars: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut heightmap: Heightmap = Heightmap::new();
    let mut start: Pos = (0, 0);
    let mut end: Pos = (0, 0);
    for row in 0..chars.len() {
        heightmap.push(Vec::new());
        for col in 0..chars[row].len() {
            match chars[row][col] {
                'S' => {
                    start = (row, col);
                    heightmap[row].push('a' as u32);
                }
                'E' => {
                    end = (row, col);
                    heightmap[row].push('z' as u32);
                }
                _ => heightmap[row].push(chars[row][col] as u32),
            }
        }
    }

    (start, end, heightmap)
}
// depth-first search over possible states
// optimise to dijkstra / A* if need be
