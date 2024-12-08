use std::fs::read_to_string;

fn count(row: usize, col: usize, grid: &[Vec<char>], rows: usize, cols: usize) -> i64{
    if col + 2 >= cols || row < 2 { return 0 }

    let mut res: i64 = 0;
    res += (grid[row][col] == 'M' && grid[row][col + 2] == 'M' && grid[row - 1][col + 1] == 'A' && grid[row - 2][col] == 'S' && grid[row - 2][col + 2] == 'S') as i64;
    res += (grid[row][col] == 'S' && grid[row][col + 2] == 'M' && grid[row - 1][col + 1] == 'A' && grid[row - 2][col] == 'S' && grid[row - 2][col + 2] == 'M') as i64;
    res += (grid[row][col] == 'S' && grid[row][col + 2] == 'S' && grid[row - 1][col + 1] == 'A' && grid[row - 2][col] == 'M' && grid[row - 2][col + 2] == 'M') as i64;
    res += (grid[row][col] == 'M' && grid[row][col + 2] == 'S' && grid[row - 1][col + 1] == 'A' && grid[row - 2][col] == 'M' && grid[row - 2][col + 2] == 'S') as i64;

    res
}

pub fn solve() -> i64 {
    let mut res: i64 = 0;

    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in read_to_string("in.txt").unwrap().lines() {
        grid.push(line.chars().collect());
    }

    let rows = grid.len();
    let cols = grid[0].len();

    for row in  0..rows {
        for col in 0..cols {
            res += count(row, col, &grid, rows, cols);
        }
    }

    res
}
