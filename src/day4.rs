use grid::Grid;
use std::fs;

pub(crate) fn day4(input_filename: &str) {
    let contents = fs::read_to_string(input_filename).expect("could not read file");
    let count = count_xmas(contents);
    println!("count: {}", count);
}

fn count_xmas(input: String) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let size_x = lines.first().expect("empty!").len();
    let size_y = lines.len();
    let mut grid = Grid::<char>::new(size_x, size_y);
    for y in 0..size_y {
        grid.insert_row(y, lines[y].chars().collect());
    }

    // Table of offsets from the current position in the grid
    // TODO: Generalise to get_offsets(k: usize)
    let searches = vec![
        mul(vec![(0, 0), (1, 1), (2, 2), (3, 3)], ( 0,  1)),  // 0 +
        mul(vec![(0, 0), (1, 1), (2, 2), (3, 3)], ( 1,  1)),  // + +
        mul(vec![(0, 0), (1, 1), (2, 2), (3, 3)], ( 1,  0)),  // + 0
        mul(vec![(0, 0), (1, 1), (2, 2), (3, 3)], (-1,  1)),  // - +
        mul(vec![(0, 0), (1, 1), (2, 2), (3, 3)], ( 0, -1)),  // 0 -
        mul(vec![(0, 0), (1, 1), (2, 2), (3, 3)], (-1, -1)),  // - -
        mul(vec![(0, 0), (1, 1), (2, 2), (3, 3)], (-1,  0)),  // - 0
        mul(vec![(0, 0), (1, 1), (2, 2), (3, 3)], ( 1, -1)),  // + -
    ];

    let sum: usize = grid
        .indexed_iter()
        .map(|((x, y), _)| {
            searches
                .iter()
                // build all possible strings from the current grid position
                .map(|search| {
                    search
                        .iter()
                        .map(|&(j, k)| grid.get(x as i64 + j, y as i64 + k))
                        .filter(|ch| ch.is_some())
                        .map(|ch| *ch.unwrap())
                        .collect::<String>()
                })
                // then count the ones that say XMAS
                .filter(|s| s == "XMAS")
                .count()
        })
        .sum();

    sum
}

fn mul(vec: Vec<(i64, i64)>, (j, k): (i64, i64)) -> Vec<(i64, i64)> {
    vec.iter().map(|&(x, y)| (x * j, y * k)).collect()
}

mod tests {
    use super::count_xmas;

    #[test]
    fn check_example() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"
        .to_string();
        assert_eq!(count_xmas(input), 18);
    }
}
