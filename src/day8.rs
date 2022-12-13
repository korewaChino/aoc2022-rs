//! Advent of Code 2022 - Day 8 - Treetop Tree House
// input is a string of numbers seperated by newline
// each number represents the height of a tree from 0 to 9
// A tree is visible if all of the other trees between it and an edge of the grid are shorter than it. Only consider trees in the same row or column; that is, only look up, down, left, or right from any given tree.
// so we make an xy grid of the input
// All of the trees around the edge of the grid are visible - since they are already on the edge, there are no trees to block the view.

pub fn generator(input: &str) -> Vec<Vec<usize>> {
    // for each line, parse each number into a usize
    let lines = input.lines();

    let mut grid = Vec::new();

    for line in lines {
        let mut row = Vec::new();
        // for char in line
        for char in line.chars() {
            // parse the char into a usize
            let num = char.to_digit(10).unwrap() as usize;
            // push the usize into the row
            row.push(num);
        }
        // push the row into the grid
        grid.push(row);
    }
    grid
}

fn row(input: &[Vec<usize>], row: usize) -> Vec<usize> {
    input[row].clone()
}

fn col(input: &[Vec<usize>], col: usize) -> Vec<usize> {
    input.iter().map(|row| row[col]).collect()
}

#[test]
fn test_part_1() {
    let input = generator(
        r#"30373
25512
65332
33549
35390"#,
    );

    assert_eq!(part_1(&input), 21);
}
pub fn part_1(input: &[Vec<usize>]) -> usize {
    // include the edge trees
    // get matrix edge trees
    // it is not a square matrix, so we need to get the max of the rows and cols
    let edge_trees = input[0].len() + input.len() - 2;
    println!("edge_trees: {}", edge_trees);

    // get visible trees when looking at each side

    // do not combine the tree heights with sum(), because we are counting visible trees, not stacking them

    let visible_top = input[0]
        .iter()
        .enumerate()
        .filter(|(i, _)| {
            // get the trees in the same row
            let row = row(input, 0);
            // get the trees in the same column
            let col = col(input, *i);
            // get the max of the row and col
            let max = row.iter().max().unwrap().max(col.iter().max().unwrap());
            // if the tree is the max, it is visible
            max == &input[0][*i]
        })
        .count();

    let visible_bottom = input[input.len() - 1]
        .iter()
        .enumerate()
        .filter(|(i, _)| {
            // get the trees in the same row
            let row = row(input, input.len() - 1);
            // get the trees in the same column
            let col = col(input, *i);
            // get the max of the row and col
            let max = row.iter().max().unwrap().max(col.iter().max().unwrap());
            // if the tree is the max, it is visible
            max == &input[input.len() - 1][*i]
        })
        .count();

    let visible_left = input
        .iter()
        .enumerate()
        .filter(|(i, _)| {
            // get the trees in the same row
            let row = row(input, *i);
            // get the trees in the same column
            let col = col(input, 0);
            // get the max of the row and col
            let max = row.iter().max().unwrap().max(col.iter().max().unwrap());
            // if the tree is the max, it is visible
            max == &input[*i][0]
        })
        .count();

    let visible_right = input
        .iter()
        .enumerate()
        .filter(|(i, _)| {
            // get the trees in the same row
            let row = row(input, *i);
            // get the trees in the same column
            let col = col(input, input[0].len() - 1);
            // get the max of the row and col
            let max = row.iter().max().unwrap().max(col.iter().max().unwrap());
            // if the tree is the max, it is visible
            max == &input[*i][input[0].len() - 1]
        })
        .count();


    // return the sum of the visible trees
    edge_trees + visible_top + visible_bottom + visible_left + visible_right
}

