use std::fs;
use std::collections::HashSet;
use rayon::prelude::*;


fn travel(grid: &str, grid_size: (i16, i16), mut pos: (i16, i16), mut dxdy: (i16, i16), block_pos: Option<(i16, i16)>) -> u16 {
    let mut visited: HashSet<(i16, i16)> = HashSet::new();

    // Garbage algo but might work in parallel
    // The blocker positions at which I was forced to change direction to 'up'
    // If I encounter these again, I know I'm stuck.
    let mut up_blockers: Vec<(i16, i16)> = vec![];

    let mut steps: i16 = 0;
    loop {
        visited.insert(pos);
        let x = pos.0 + dxdy.0;
        let y = pos.1 + dxdy.1;
        if (x < 0) || (y < 0) || (x >= grid_size.0) || (y >= grid_size.1) {
            break;
        } else if grid.lines().nth(x as usize).unwrap().chars().nth(y as usize).unwrap() == '#' {
            dxdy = (dxdy.1, -dxdy.0);
            if dxdy.0 == -1 && dxdy.1 == 0 {
                if (up_blockers.iter().any(|&up_blocker| up_blocker == (x, y))) {
                    return 0;
                } else {
                    up_blockers.push((x, y));
                }
            }
        } else {
            match block_pos {
                Some(block_pos_) => {
                    if block_pos_.0 == x && block_pos_.1 == y {
                        dxdy = (dxdy.1, -dxdy.0);
                        if dxdy.0 == -1 && dxdy.1 == 0 {
                            if up_blockers.iter().any(|&up_blocker| up_blocker == (x, y)) {
                                return 0;
                            } else {
                                up_blockers.push((x, y));
                            }
                        }
                    } else {
                        pos = (x, y);
                    }
                },
                None => {
                    pos = (x, y);
                }
            }
        }
        steps += 1;
    }

    visited.len() as u16
}


fn visited_positions(grid: &str) -> u16 {

    let n_rows = grid.lines().count() as i16;
    let n_cols = grid.lines().nth(0).unwrap().len() as i16;

    // origin is top left, so ^ indicates an initial travel direction of (-1, 0)
    let dxdy: (i16, i16) = (-1, 0);
    let mut pos: (i16, i16) = (0, 0);
    for (i, line) in grid.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '^' {
                pos = (i as i16, j as i16);
                break;
            }
        }
    }

    travel(grid, (n_rows, n_cols), pos, dxdy, None)
}

fn find_blocking_positions(grid: &str) -> i16 {

    let n_rows = grid.lines().count() as i16;
    let n_cols = grid.lines().nth(0).unwrap().len() as i16;

    // origin is top left, so ^ indicates an initial travel direction of (-1, 0)
    let dxdy: (i16, i16) = (-1, 0);
    let mut pos: (i16, i16) = (0, 0);
    for (i, line) in grid.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '^' {
                pos = (i as i16, j as i16);
                break;
            }
        }
    }

    let rows: Vec<i16> = (0..n_rows).collect();
    let result = rows.par_iter().map(
        |&row| {
            let mut result: i16 = 0;
            for col in 0..n_cols {
                let blocking_pos = (row, col);
                let n_steps = travel(grid, (n_rows, n_cols), pos, dxdy, Some(blocking_pos));
                if n_steps == 0 {
                    result += 1;
                }
            }
            result
        }
    ).reduce(|| 0, |a, b| a + b);
    result
}

fn main() {
    let grid = fs::read_to_string("input6_1.txt").unwrap();
    let result = find_blocking_positions(grid.as_str());
    print!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {

        let input: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let result = visited_positions(input);
        assert_eq!(result, 41);
    }

    #[test]
    fn it_works2() {

        let input: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let result = find_blocking_positions(input);
        assert_eq!(result, 6);
    }
}