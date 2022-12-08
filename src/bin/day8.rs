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

    let trees_visible = visibility_map.into_iter().flatten().filter(|b| *b).count();
    println!("{} trees visible from outside grid", trees_visible);

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
