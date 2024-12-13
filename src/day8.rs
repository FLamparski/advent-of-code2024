use std::{
    collections::{HashMap, HashSet},
    fs,
};

use grid::Grid;

pub(crate) fn day8(input_filename: &str) {
    let contents = fs::read_to_string(input_filename).expect("could not read file");
    let map = parse_antenna_map(&contents);
    let antennas = find_antennas(&map);

    let map_bounds = (
        0..(map.size().0 as i64),
        0..(map.size().1 as i64),
    );

    let antinodes = antennas
        .iter()
        .map(|(_, antennas)| compute_antinodes(antennas))
        .reduce(|a1, a2| {
            let set: HashSet<_> = a1.union(&a2).map(|p| p.to_owned()).collect();
            set
        })
        .unwrap()
        .iter()
        .filter(|(x, y)| map_bounds.0.contains(x) && map_bounds.1.contains(y))
        .count();
    
    println!("there are {} antinodes.", antinodes);
}

fn compute_antinodes(antennas: &Vec<(usize, usize)>) -> HashSet<(i64, i64)> {
    let mut antinodes = HashSet::<(i64, i64)>::new();

    let pairs = pairs(&antennas);
    for pair in pairs {
        // compute line equation through the two points
        let (x1, y1) = (pair.0 .0 as f64, pair.0 .1 as f64);
        let (x2, y2) = (pair.1 .0 as f64, pair.1 .1 as f64);
        let (a, b) = if x1 == x2 {
            (0.0, 0.0)
        } else {
            let a = (y2 - y1) / (x2 - x1);
            let b = y1 - a * x1;
            (a, b)
        };
        let distance_x = (x2 - x1).abs();
        let anti1_x = x1 + 2.0 * distance_x;
        let anti1_y = anti1_x * a + b;
        let anti2_x = x2 - 2.0 * distance_x;
        let anti2_y = anti2_x * a + b;

        antinodes.insert((anti1_x.round() as i64, anti1_y.round() as i64));
        antinodes.insert((anti2_x.round() as i64, anti2_y.round() as i64));
    }

    antinodes
}

fn parse_antenna_map(input: &str) -> Grid<char> {
    let lines = input.lines().collect::<Vec<_>>();
    let size_x = lines.first().expect("empty!").len();
    let size_y = lines.len();
    let mut map = Grid::<char>::new(size_x, size_y);

    for ((x, y), ch) in map.indexed_iter_mut() {
        let c = &lines[y][x..x + 1];
        *ch = c.chars().last().unwrap();
    }

    map
}

fn find_antennas(map: &Grid<char>) -> HashMap<char, Vec<(usize, usize)>> {
    let mut antenna_index = HashMap::<char, Vec<(usize, usize)>>::new();

    map.indexed_iter()
        .filter(|&(_, ch)| ch != &'.')
        .for_each(|(coords, ch)| {
            if !antenna_index.contains_key(ch) {
                antenna_index.insert(*ch, vec![]);
            }
            antenna_index.get_mut(ch).unwrap().push(coords);
        });

    antenna_index
}

fn pairs<T: PartialEq>(v: &Vec<T>) -> Vec<Pair<&T>> {
    let mut vout = Vec::<Pair<&T>>::new();

    for i in 0..v.len() {
        for j in 0..v.len() {
            if i != j {
                let pair = Pair(&v[i], &v[j]);
                if vout.iter().all(|p| *p != pair) {
                    vout.push(pair);
                }
            }
        }
    }

    vout
}

#[derive(Debug)]
struct Pair<T>(T, T);

impl<T> PartialEq for Pair<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
    }
}

mod tests {
    use std::collections::HashSet;

    use super::{compute_antinodes, find_antennas, pairs, parse_antenna_map, Pair};

    #[test]
    fn check_input() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";
        let map = parse_antenna_map(input);
        assert_eq!(map.get(8, 1).unwrap(), &'0');

        let antennas = find_antennas(&map);
        println!("{:#?}", antennas);

        let pairs_of_A = pairs(antennas.get(&'A').unwrap());
        println!("{:?}", pairs_of_A);
    }

    #[test]
    fn check_pairs() {
        let v = vec![1, 2, 3];
        let ps = pairs(&v);
        assert_eq!(ps, vec![Pair(&1, &2), Pair(&1, &3), Pair(&3, &2)]);

        assert_eq!(Pair(1, 2), Pair(2, 1));
    }

    #[test]
    fn check_antinodes() {
        let input = "..........
...#......
..........
....a.....
..........
.....a....
..........
......#...
..........
..........
";
        let map = parse_antenna_map(input);
        let antennas = find_antennas(&map);

        let antinode_positions = antennas
            .get(&'#')
            .unwrap()
            .iter()
            .map(|&(x, y)| (x as i64, y as i64))
            .collect::<HashSet<_>>();
        let antinodes = compute_antinodes(antennas.get(&'a').unwrap());
        assert_eq!(antinodes, antinode_positions);
    }

    #[test]
    fn check_antinodes_3() {
        let input = "..........
...#......
#.........
....a.....
........a.
.....a....
..#.......
......#...
..........
..........
";
        let map = parse_antenna_map(input);
        let antennas = find_antennas(&map);
        let antinode_positions = antennas
            .get(&'#')
            .unwrap()
            .iter()
            .map(|&(x, y)| (x as i64, y as i64))
            .collect::<HashSet<_>>();
        let antinodes = compute_antinodes(antennas.get(&'a').unwrap());
        println!("found : {:?}", antinodes);
        println!("expect: {:?}", antinode_positions);
        let num_in_bounds_antinodes = antinodes.intersection(&antinode_positions).count();
        assert_eq!(num_in_bounds_antinodes, 4);
    }
}
