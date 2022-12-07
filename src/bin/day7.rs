use std::collections::HashMap;

use aoc22::input::read_input;

#[derive(Debug)]
struct File {
    size: usize,
}

#[derive(Debug)]
struct DirectoryContents {
    files: Vec<File>,
    sub_directories: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input("day7")?;

    let file_system: HashMap<String, DirectoryContents> = parse_filesystem(input);

    let mut total_sum = 0;
    for k in file_system.keys() {
        let sum: usize = directory_size(k, &file_system, true);
        if sum <= 100000 {
            total_sum += sum;
        }
    }
    println!("sum of small dirs {}", total_sum);

    let total_used_space = directory_size("/", &file_system, true);
    let remaining_space = 70000000 - total_used_space;
    let space_needed = 30000000 - remaining_space;
    let size_of_smallest_possible_dir = file_system
        .iter()
        .map(|(k, _)| {
            vec![
                directory_size(k, &file_system, true),
                directory_size(k, &file_system, false),
            ]
        })
        .flatten()
        .filter(|size| *size >= space_needed)
        .min()
        .unwrap();
    println!(
        "size of smallest dir to delete {}",
        size_of_smallest_possible_dir
    );

    Ok(())
}

fn directory_size(
    directory: &str,
    file_system: &HashMap<String, DirectoryContents>,
    recurse: bool,
) -> usize {
    let dir = file_system.get(directory).unwrap();
    let file_sum = dir.files.iter().map(|f| f.size).sum::<usize>();
    if !recurse {
        return file_sum;
    }

    file_sum
        + dir
            .sub_directories
            .iter()
            .map(|d| directory_size(d, file_system, true))
            .sum::<usize>()
}

fn parse_filesystem(input: String) -> HashMap<String, DirectoryContents> {
    let mut directories: HashMap<String, DirectoryContents> = HashMap::new();

    let current_dir = &mut "/".to_string();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(" ").collect();

        // This could probably be cleaner but parses commands
        match parts[1] {
            "cd" => {
                if parts[2] == ".." {
                    let last_slash_index = current_dir.clone().rfind("/").unwrap();
                    current_dir.replace_range(last_slash_index.., "");
                } else {
                    current_dir.push_str(&format!("/{}", parts[2]));
                }
                continue;
            }
            "ls" => continue,
            _ => (),
        }

        // Parse files / subdirs if it wasn't a command
        let dir_contents = directories
            .entry(current_dir.clone())
            .or_insert(DirectoryContents {
                files: vec![],
                sub_directories: vec![],
            });

        if parts[0] == "dir" {
            dir_contents
                .sub_directories
                .push(format!("{}/{}", current_dir, parts[1]));
        } else if let Ok(size) = parts[0].parse::<usize>() {
            dir_contents.files.push(File { size: size });
        }
    }

    directories
}
