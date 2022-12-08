use crate::lib::load_file::load_data_file;

use super::ExecuteResponse;

//          visible, height, scenic(top, left, bottom, right)
type Tree = (bool, i32, (usize, usize, usize, usize));

pub fn execute() -> ExecuteResponse {
    let data = load_data_file("day_8.txt")?;

    let mut trees: Vec<Vec<Tree>> = vec![];
    let mut chars: Vec<Vec<i32>> = vec![];

    // Make tree structure as following
    // [
    //   [(false, 23),(false, 12),(false, 5),(false, 26),(false, 57)]
    //   [(false, 23),(false, 12),(false, 5),(false, 26),(false, 57)]
    // ]
    data.lines()
        .into_iter()
        .enumerate()
        .for_each(|(v_index, line)| {
            let heights = line.chars().map(|c| c.to_string().parse::<i32>().unwrap());

            trees.push(
                heights
                    .clone()
                    .map(|height| (false, height, (0, 0, 0, 0)))
                    .collect(),
            );
            chars.push(heights.clone().collect());

            let mut max_height = 0;
            // Each row horizontally
            heights
                .clone()
                .into_iter()
                .enumerate()
                .for_each(|(index, height)| {
                    // Trees on side are visible
                    if index.eq(&0) || index.eq(&(&line.len() - 1)) {
                        trees[v_index][index].0 = true;
                    }

                    if height > max_height {
                        trees[v_index][index].0 = true;
                        max_height = height;
                    }
                });

            // Reverse heights to check from R to L
            max_height = 0;
            heights
                .clone()
                .rev()
                .enumerate()
                .for_each(|(index, height)| {
                    let rev_index = &line.len() - index - 1;
                    if height > max_height {
                        trees[v_index][rev_index].0 = true;
                        max_height = height;
                    }
                });
        });

    // Check trees vertically
    let mut vertical: Vec<Vec<i32>> = vec![];
    let mut vertical_index = 0usize;

    for row in chars.iter() {
        row.iter().for_each(|height| {
            if let Some(existing) = vertical.get_mut(vertical_index) {
                existing.push(*height);
            } else {
                vertical.push(vec![*height]);
            }

            vertical_index += 1;
        });

        vertical_index = 0;
    }

    vertical
        .clone()
        .into_iter()
        .enumerate()
        .for_each(|(vert_index, mut row)| {
            let mut max_height = -1;
            for (horiz_index, height) in row.iter().enumerate() {
                if height > &max_height {
                    trees[horiz_index][vert_index].0 = true;
                    max_height = *height;
                }
            }

            row.reverse();
            max_height = -1;

            for (horiz_index, height) in row.iter().enumerate() {
                let rev_index = &row.len() - horiz_index - 1;
                if height > &max_height {
                    trees[rev_index][vert_index].0 = true;
                    max_height = *height;
                }
            }
        });

    let cloned_trees = trees.clone();

    // Calculate scenic view
    for (horizontal_index, tree_line) in cloned_trees.iter().enumerate() {
        for (i, tree) in tree_line.iter().enumerate() {
            let tree_neighbors_left = &tree_line[..i];

            for x in (0..tree_neighbors_left.len()).rev() {
                let neighbor = tree_neighbors_left[x];
                if neighbor.1 == tree.1 || neighbor.1 > tree.1 {
                    trees[horizontal_index][i].2 .3 += 1;
                    break;
                }

                trees[horizontal_index][i].2 .3 += 1;
            }

            let tree_neighbors_right = &tree_line[i + 1..];

            for neighbor in tree_neighbors_right {
                if neighbor.1 == tree.1 || neighbor.1 > tree.1 {
                    trees[horizontal_index][i].2 .1 += 1;
                    break;
                }

                trees[horizontal_index][i].2 .1 += 1;
            }

            if horizontal_index > 0 {
                let trees_above: Vec<Tree> = cloned_trees
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| i < &horizontal_index)
                    .map(|(_, t)| t[i])
                    .rev()
                    .collect();

                for neighbor in trees_above {
                    if neighbor.1 == tree.1 || neighbor.1 > tree.1 {
                        trees[horizontal_index][i].2 .0 += 1;
                        break;
                    }

                    trees[horizontal_index][i].2 .0 += 1;
                }

                let trees_below: Vec<Tree> = cloned_trees
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| i > &horizontal_index)
                    .map(|(_, t)| t[i])
                    .collect();

                for neighbor in trees_below {
                    if neighbor.1 == tree.1 || neighbor.1 > tree.1 {
                        trees[horizontal_index][i].2 .2 += 1;
                        break;
                    }

                    trees[horizontal_index][i].2 .2 += 1;
                }
            }
        }
    }

    let visible: Vec<&Tree> = trees
        .iter()
        .flat_map(|t| t)
        .filter(|t| t.0 == true)
        .collect();

    let high_scenic: usize = trees
        .iter()
        .flat_map(|line| {
            line.iter()
                .map(|tree| tree.2 .0 * tree.2 .1 * tree.2 .2 * tree.2 .3)
        })
        .max()
        .unwrap();

    println!("part 1 - {}", visible.len());
    println!("part 2 - {}", high_scenic);

    Ok(())
}
