use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct GuardPosition {
    row_num: i64,
    col_num: i64,
    direction: char
}

impl GuardPosition {
    fn move_up(&mut self) {
        self.row_num -= 1;
    }

    fn move_down(&mut self) {
        self.row_num += 1;
    }

    fn move_right(&mut self) {
        self.col_num += 1;
    }

    fn move_left(&mut self) {
        self.col_num -= 1;
    }

    fn move_self(&mut self) {
        if self.direction == '^' {
            self.move_up()
        }

        if self.direction == '>' {
            self.move_right()
        }

        if self.direction == 'V' {
            self.move_down()
        }

        if self.direction == '<' {
            self.move_left()
        }
    }

    fn rotate_clockwise(&mut self) {
        if self.direction == '^' {
            self.direction = '>';
        } else if self.direction == '>' {
            self.direction = 'V';
        } else if self.direction == 'V' {
            self.direction = '<';
        } else if self.direction == '<' {
            self.direction = '^';
        }
    }

    fn is_within_bounds(&self, num_rows: i64, num_cols: i64) -> bool {
        self.row_num >= 0 && self.row_num < num_rows && self.col_num >= 0 && self.col_num < num_cols
    }

    fn is_inside_obstacle(&self, grid: &[Vec<char>]) -> bool {
        grid[self.row_num as usize][self.col_num as usize] == '#'
    }
}

fn find_guard(grid: &[Vec<char>]) -> Option<GuardPosition>{
    for (row_num, row) in grid.iter().enumerate(){
        for (col_num, cell) in row.iter().enumerate() {
            if *cell == '^' || *cell == '>' || *cell == 'V' || *cell == '<' {
                let guard_position = GuardPosition {
                    row_num: row_num as i64,
                    col_num: col_num as i64,
                    direction: *cell
                };

                return Some(guard_position);
            }
        }
    }

    None
}

fn move_guard(grid: &[Vec<char>], guard_position: GuardPosition) -> Option<GuardPosition> {
    let mut new_position: GuardPosition = guard_position.clone();
    new_position.move_self();

    let num_rows = grid.len() as i64;
    let num_cols = grid[0].len() as i64;
    if !new_position.is_within_bounds(num_rows, num_cols) {
        return None;
    }

    if new_position.is_inside_obstacle(grid) {
        new_position = guard_position.clone();
        new_position.rotate_clockwise();
        return Some(new_position);
    }
    
    Some(new_position)
}

fn has_cycle(grid: &[Vec<char>], max_iterations: i64) -> bool {
    let mut guard_position = find_guard(grid);
    let mut iterations: i64 = 0;
    while guard_position.is_some() {
        guard_position = move_guard(grid, guard_position.unwrap());
        iterations += 1;
        
        if iterations == max_iterations {
            return true;
        }
    }
    
    false
}

pub fn solve() -> i64 {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in read_to_string("in.txt").unwrap().lines() {
        grid.push(line.chars().collect());
    }
    
    let mut res: i64 = 0;
    let height = grid.len();
    let width = grid[0].len();
    for row_num in 0..height {
        for col_num in 0..width {
            if grid[row_num][col_num] == '.' {
                grid[row_num][col_num] = '#';
                let max_iterations = (height as i64) * (width as i64) * 4;
                res += has_cycle(&grid, max_iterations) as i64;
                grid[row_num][col_num] = '.';
            }
        }
    }
    res
}
