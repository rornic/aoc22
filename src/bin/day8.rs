use aoc22::input::read_input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input("day8")?;

    let mut heightmap: Vec<Vec<u32>> = vec![];
    input.lines().for_each(|line| {
        heightmap.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
    });

    let rows = heightmap.len();
    let cols = heightmap[0].len();

    // Construct visibility map from heightmap
    let mut visibility_map: Vec<Vec<bool>> = vec![];
    for row in 0..rows {
        visibility_map.push(vec![]);
        for col in 0..cols {
            visibility_map[row].push(is_visible(row, col, &heightmap));
        }
    }

    let trees_visible = visibility_map.iter().flatten().filter(|b| **b).count();
    println!("{} trees visible from outside grid", trees_visible);

    let mut scenic_scores: Vec<u32> = vec![];
    for row in 0..rows {
        for col in 0..cols {
            scenic_scores.push(scenic_score(row, col, &heightmap, &visibility_map));
        }
    }

    let highest_scenic_score = scenic_scores.iter().max().unwrap();
    println!("highest scenic score is {}", highest_scenic_score);

    Ok(())
}

fn is_visible(row: usize, col: usize, heightmap: &Vec<Vec<u32>>) -> bool {
    let height = heightmap[row][col];
    let rows = heightmap.len();
    let cols = heightmap[0].len();

    // Exit early if on edge of heightmap
    if row == 0 || row == rows - 1 || col == 0 || col == cols - 1 {
        return true;
    }

    // Inefficiency here: should exit early when `visibility` becomes true to avoid
    // unnecessary checks
    let mut visible = false;
    visible |= (0..row).into_iter().all(|i| heightmap[i][col] < height);
    visible |= (row + 1..rows)
        .into_iter()
        .all(|i| heightmap[i][col] < height);
    visible |= (0..col).into_iter().all(|i| heightmap[row][i] < height);
    visible |= (col + 1..cols)
        .into_iter()
        .all(|i| heightmap[row][i] < height);
    visible
}

fn scenic_score(
    row: usize,
    col: usize,
    heightmap: &Vec<Vec<u32>>,
    visibility_map: &Vec<Vec<bool>>,
) -> u32 {
    if !visibility_map[row][col] {
        return 1;
    }

    let rows = heightmap.len();
    let cols = heightmap[0].len();

    let val = heightmap[row][col];

    // up
    let mut furthest = 0;
    for i in (0..row).rev() {
        furthest = row - i;
        if heightmap[i][col] >= val {
            break;
        }
    }
    let mut score = furthest;

    // down
    furthest = 0;
    for i in row + 1..rows {
        furthest += 1;
        if heightmap[i][col] >= val {
            break;
        }
    }
    score *= furthest;

    // left
    furthest = 0;
    for i in (0..col).rev() {
        furthest = col - i;
        if heightmap[row][i] >= val {
            break;
        }
    }
    score *= furthest;

    // right
    furthest = 0;
    for i in col + 1..cols {
        furthest = i - col;
        if heightmap[row][i] >= val {
            break;
        }
    }
    score *= furthest;

    score as u32
}
