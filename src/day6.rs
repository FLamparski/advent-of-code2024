use std::{collections::HashSet, fmt::{Display, Write}, fs};

use grid::Grid;

pub(crate) fn day6(input_filename: &str) {
    let contents = fs::read_to_string(input_filename).expect("could not read file");
    let map = parse_map(contents);
    let n_unique_positions = find_exit(map);
    println!("unique positions: {}", n_unique_positions);
}

fn find_exit(map: TileMap) -> usize {
    let mut map = map;
    let mut visited = HashSet::<(usize, usize)>::new();
    visited.insert(map.get_guard_position());
    loop {
        // println!("move {}:\n{}\n", visited.len(), map);
        let next = get_next_guard_position(&map);
        if let Some((next_pos, direction)) = next {
            visited.insert(next_pos);
            map.move_guard(next_pos, direction);
        } else {
            return visited.len();
        }
    }
}

fn parse_map(input: String) -> TileMap {
    let lines = input.lines().collect::<Vec<_>>();
    let size_x = lines.first().expect("empty!").len();
    let size_y = lines.len();
    let mut map = TileMap(Grid::new(size_x, size_y));

    for ((x, y), tile) in map.0.indexed_iter_mut() {
        let ch = &lines[y][x..x+1];
        *tile = match ch {
            "." => TileType::Path,
            "#" => TileType::Obstacle,
            "^" => TileType::Guard(Direction::Up),
            ">" => TileType::Guard(Direction::Right),
            "<" => TileType::Guard(Direction::Left),
            "v" => TileType::Guard(Direction::Down),
            _ => panic!("unknown tile: {}", ch),
        };
    }

    map
}

fn get_next_guard_position(map: &TileMap) -> Option<((usize, usize), Direction)> {
    let (guard_x, guard_y) = map.get_guard_position();
    let (facing_pos,direction, tile) = map.sense_guard();
    match tile {
        Some(TileType::Path) => Some((facing_pos, direction)),
        Some(TileType::Obstacle) => {
            let turned_dir = direction.turn_right();
            let next_pos = match turned_dir {
                Direction::Up => (guard_x, guard_y - 1),
                Direction::Left => (guard_x - 1, guard_y),
                Direction::Down => (guard_x, guard_y + 1),
                Direction::Right => (guard_x + 1, guard_y),
            };
            Some((next_pos, turned_dir))
        },
        None => None,
        _ => panic!()
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(PartialEq, Debug)]
enum TileType {
    Guard(Direction),
    Obstacle,
    Path,
}

impl Default for TileType {
    fn default() -> Self {
        TileType::Path
    }
}

struct TileMap(Grid<TileType>);

impl<'a> Display for TileMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for col in self.0.iter_cols() {
            for tile in col {
                let ch = match *tile {
                    TileType::Path => '.',
                    TileType::Obstacle => '#',
                    TileType::Guard(Direction::Up) => '^',
                    TileType::Guard(Direction::Left) => '<',
                    TileType::Guard(Direction::Down) => 'v',
                    TileType::Guard(Direction::Right) => '>',
                };
                f.write_char(ch)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl TileMap {
    fn get_guard_position(&self) -> (usize, usize) {
        let (guard_pos, _) = self.0.indexed_iter().find(|&(_, tile)| match tile {
            TileType::Guard(..) => true,
            _ => false
        }).expect("guard not on map!");
        guard_pos    
    }

    fn sense_guard(&self) -> ((usize, usize), Direction, Option<&TileType>) {
        let (guard_x, guard_y) = self.get_guard_position();
        let guard_tile = self.0.get(guard_x, guard_y).unwrap();
        let (sense_x, sense_y) = match guard_tile {
            TileType::Guard(Direction::Up) => (guard_x, guard_y - 1),
            TileType::Guard(Direction::Left) => (guard_x - 1, guard_y),
            TileType::Guard(Direction::Down) => (guard_x, guard_y + 1),
            TileType::Guard(Direction::Right) => (guard_x + 1, guard_y),
            _ => panic!("no guard at guard_pos!")
        };
        if let TileType::Guard(direction) = guard_tile {
            ((sense_x, sense_y), direction.clone(), self.0.get(sense_x, sense_y))
        } else {
            panic!();
        }
    }

    fn move_guard(&mut self, (new_x, new_y): (usize, usize), direction: Direction) {
        let (cur_x, cur_y) = self.get_guard_position();
        let mut cur_row = self.0.remove_row(cur_x).unwrap();
        cur_row[cur_y] = TileType::Path;
        self.0.insert_row(cur_x, cur_row);
        let mut new_row = self.0.remove_row(new_x).unwrap();
        new_row[new_y] = TileType::Guard(direction);
        self.0.insert_row(new_x, new_row);
    }
}

mod tests {
    use grid::Grid;

    use crate::day6::{get_next_guard_position, Direction, TileType};

    use super::{find_exit, parse_map, TileMap};

    #[test]
    fn check_example_map() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
        let map = parse_map(input.to_string());
        let formatted = format!("{}", map);
        assert_eq!(*map.0.get(4, 6).unwrap(), TileType::Guard(Direction::Up));
        assert_eq!(formatted, input);
    }

    #[test]
    fn moving_stuff() {
        let input = ">#\n.#";
        let map = parse_map(input.to_string());
        let next = get_next_guard_position(&map);
        assert!(next.is_some());
        let (next_pos, direction) = next.unwrap();
        assert_eq!(next_pos, (0, 1));
        assert_eq!(direction, Direction::Down);
    }

    #[test]
    fn find_exit_2x2() {
        let input = ">#\n.#";
        let map = parse_map(input.to_string());
        let num_moves = find_exit(map);
        assert_eq!(num_moves, 2);
    }

    #[test]
    fn find_exit_example() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
        let map = parse_map(input.to_string());
        let num_moves = find_exit(map);
        assert_eq!(num_moves, 41);
    }
}