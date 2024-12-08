use std::fs;

fn find(grid: &str, grid_size: (i16, i16), needle: &str, pos: (i16, i16), dxdy: (i16, i16)) -> bool {
    let mut i: i16 = 0;
    loop {
        let x = pos.0 + dxdy.0*i;
        let y = pos.1 + dxdy.1*i;
        if (x < 0) || (y < 0) || (x >= grid_size.0) || (y >= grid_size.1) || (grid.lines().nth(x as usize).unwrap().chars().nth(y as usize) != needle.chars().nth(i as usize)) {
            return false;
        } else {
            i += 1;
            if i==needle.len() as i16 {
                return true;
            }
        }
    }
}

fn find_x(grid: &str) -> u16 {
    let n_rows = grid.lines().count() as i16;
    let n_cols = grid.lines().nth(0).unwrap().len() as i16;
    let mut n_results: u16 = 0;

    for i in 0..n_rows {
        for j in 0..n_cols {
            let mut found_mas = find(grid, (n_rows, n_cols), "MAS", (i, j), (1, 1));
            let mut found_sam = find(grid, (n_rows, n_cols), "SAM", (i, j), (1, 1));
            let arm1_found = found_mas || found_sam;

            found_mas = find(grid, (n_rows, n_cols), "MAS", (i, j+2), (1, -1));
            found_sam = find(grid, (n_rows, n_cols), "SAM", (i, j+2), (1, -1));
            let arm2_found = found_mas || found_sam;

            if arm1_found && arm2_found {
                n_results += 1;
            }

        }
    }
    n_results
}

fn find_words(grid: &str, needle: &str) -> u16 {

    let mut dxdy: Vec<(i16, i16)> = vec![];
    let deltas = [-1, 0, 1];
    for dx in deltas {
        for dy in deltas {
            if !(dx == 0 && dy == 0) {
                dxdy.push((dx, dy));
            }
        }
    }

    let n_rows = grid.lines().count() as i16;
    let n_cols = grid.lines().nth(0).unwrap().len() as i16;
    let mut n_results: u16 = 0;

    for i in 0..n_rows {
        for j in 0..n_cols {
            for (dx, dy) in &dxdy {
                n_results += find(grid, (n_rows, n_cols), needle, (i, j), (*dx, *dy)) as u16;
            }
        }
    }

    n_results
}

fn main() {
    let grid = fs::read_to_string("input4_1.txt").unwrap();
    let result_a = find_words(grid.as_str(), "XMAS");
    print!("{:?}\n", result_a);
    let result_b = find_x(grid.as_str());
    print!("{:?}\n", result_b);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {

        let input: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let result = find_words(input, "XMAS");
        assert_eq!(result, 18);
    }

    #[test]
    fn it_works2() {

        let input: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let result = find_x(input);
        assert_eq!(result, 9);
    }
}