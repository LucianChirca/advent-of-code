use std::fs::read_to_string;

fn count_horizontal(row: usize, col: usize, grid: &[Vec<char>], rows: usize, cols: usize) -> i64{
    let mut res: i64 = 0;
    res += (col + 3 < cols && grid[row][col] == 'X' && grid[row][col + 1] == 'M' && grid[row][col + 2] == 'A' && grid[row][col + 3] == 'S') as i64;
    // Now in reverse
    res += (col >= 3 && grid[row][col] == 'X' && grid[row][col - 1] == 'M' && grid[row][col - 2] == 'A' && grid[row][col - 3] == 'S') as i64;

    res
}

fn count_vertical(row: usize, col: usize, grid: &[Vec<char>], rows: usize, cols: usize) -> i64{
    let mut res: i64 = 0;
    res += (row + 3 < rows && grid[row][col] == 'X' && grid[row + 1][col] == 'M' && grid[row + 2][col] == 'A' && grid[row + 3][col] == 'S') as i64;
    // Now in reverse
    res += (row >= 3 && grid[row][col] == 'X' && grid[row - 1][col] == 'M' && grid[row - 2][col] == 'A' && grid[row - 3][col] == 'S') as i64;

    res
}

fn count_diagonal(row: usize, col: usize, grid: &[Vec<char>], rows: usize, cols: usize) -> i64{
    let mut res: i64 = 0;
    // In the / direction
    res += (row + 3 < rows && col + 3 < cols && grid[row][col] == 'X' && grid[row + 1][col + 1] == 'M' && grid[row + 2][col + 2] == 'A' && grid[row + 3][col + 3] == 'S') as i64;
    // Now in reverse
    res += (row >= 3 && col >= 3 && grid[row][col] == 'X' && grid[row - 1][col - 1] == 'M' && grid[row - 2][col - 2] == 'A' && grid[row - 3][col - 3] == 'S') as i64;

    // In the \ direction
    res += (row >= 3 && col + 3 < cols && grid[row][col] == 'X' && grid[row - 1][col + 1] == 'M' && grid[row - 2][col + 2] == 'A' && grid[row - 3][col + 3] == 'S') as i64;
    // Now in reverse
    res += (row + 3 < rows && col >= 3 && grid[row][col] == 'X' && grid[row + 1][col - 1] == 'M' && grid[row + 2][col - 2] == 'A' && grid[row + 3][col - 3] == 'S') as i64;


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
            res += count_horizontal(row, col, &grid, rows, cols);
            res += count_vertical(row, col, &grid, rows, cols);
            res += count_diagonal(row, col, &grid, rows, cols);
        }
    }
    
    res
}
