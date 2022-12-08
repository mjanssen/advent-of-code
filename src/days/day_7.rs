use crate::lib::load_file::load_data_file;

use super::ExecuteResponse;

pub fn execute() -> ExecuteResponse {
    let data = load_data_file("day_7.txt")?;

    let mut tree = vec![("root", 0)];

    data.lines()
        .filter(|line| !line.is_empty())
        .filter(|line| !line.starts_with("$ ls"))
        .filter(|line| !line.starts_with("dir"))
        .for_each(
            |line| match line.split_whitespace().collect::<Vec<&str>>()[..] {
                ["$", "cd", ".."] => {
                    let (name, size) = tree.pop().unwrap();

                    let i = &tree.len() - 1;

                    tree[i].1 += size;
                    tree.insert(1, (name, size));
                }
                ["$", "cd", name] => {
                    tree.push((name, 0));
                }
                [size, _fn] => {
                    let i = &tree.len() - 1;
                    let parsed_size = size.parse::<i32>().unwrap_or(0);

                    tree[i].1 += parsed_size;
                    tree[0].1 += parsed_size;
                }
                _ => {}
            },
        );

    let total_space = 70_000_000;
    let total_required_space = 30_000_000;

    // Root directory
    let (_, size) = tree[0];

    let space_required = total_required_space - (total_space - size);

    println!("Required space: {}", space_required);

    let mut possible_delete: Vec<&(&str, i32)> = tree
        .iter()
        .filter(|d| d.0 != "/" && d.1 >= space_required)
        .collect();

    possible_delete.sort_by(|a, b| a.1.cmp(&b.1));

    println!("part 2 - {:?}", possible_delete[0]);

    let filtered_tree: Vec<(&str, i32)> = tree
        .into_iter()
        .filter(|(name, size)| name != &"/" && size <= &100_000)
        .map(|i| i)
        .collect();

    let sum: &i32 = &filtered_tree.iter().map(|(_, size)| size).sum();

    println!("part 1 - {}", sum);

    Ok(())
}
