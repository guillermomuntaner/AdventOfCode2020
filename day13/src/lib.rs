use std::collections::HashSet;
use array2d::Array2D;

pub fn part1(input: &str) -> usize {

    let (coordinates_str, instructions_str) = input.split_once("\n\n").unwrap();

    let instructions = instructions_str
        .lines()
        .take(1)
        .map(|line| line.split_once('=').unwrap())
        .map(|(command, position_str)| (command, position_str.parse::<usize>().unwrap()))
        .collect::<Vec<_>>();

    let coordinates = coordinates_str.lines()
        .map(|coordinate| coordinate.split_once(',').unwrap())
        .map(|(x, y)| (y.parse::<usize>().unwrap(), x.parse::<usize>().unwrap()));

    let mut folded_coordinates: HashSet<(usize, usize)> = HashSet::new();
    for (mut y, mut x) in coordinates {
        for &(command, position) in &instructions {
            match command {
                "fold along x" => {
                    if x == position || x > 2 * position {
                        continue
                    } else if x > position {
                        x = 2 * position - x
                    }
                },
                "fold along y" => {
                    if y == position || y > 2 * position {
                        continue
                    } else if y > position {
                        y = 2 * position - y
                    }
                },
                _ => panic!("Unexpected instruction {}", command)
            }
        }
        folded_coordinates.insert((y, x));
    }

    folded_coordinates.len()
}

pub fn part2(input: &str) -> String {

    let (coordinates_str, instructions_str) = input.split_once("\n\n").unwrap();

    let instructions = instructions_str
        .lines()
        .map(|line| line.split_once('=').unwrap())
        .map(|(command, position_str)| (command, position_str.parse::<usize>().unwrap()))
        .collect::<Vec<_>>();

    let coordinates = coordinates_str.lines()
        .map(|coordinate| coordinate.split_once(',').unwrap())
        .map(|(x, y)| (y.parse::<usize>().unwrap(), x.parse::<usize>().unwrap()));

    let mut width = 0;
    let mut height = 0;
    let mut folded_coordinates: Vec<(usize, usize)> = Vec::new();

    for (mut y, mut x) in coordinates {
        for &(command, position) in &instructions {
            match command {
                "fold along x" => {
                    if x == position || x > 2 * position {
                        continue
                    } else if x > position {
                        x = 2 * position - x
                    }
                    width = position
                },
                "fold along y" => {
                    if y == position || y > 2 * position {
                        continue
                    } else if y > position {
                        y = 2 * position - y
                    }
                    height = position
                },
                _ => panic!("Unexpected instruction {}", command)
            }
        }
        folded_coordinates.push((y, x))
    }

    let mut map = Array2D::<char>::filled_with('.', height, width);
    folded_coordinates.iter().for_each(|&folded_coordinate| map[folded_coordinate] = '#');

    map.rows_iter()
        .map(|row| row.collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
        assert_eq!(crate::part1(input), 17);
    }

    #[test]
    fn test_part2() {
        let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
        let output = "\
        #####\n\
        #...#\n\
        #...#\n\
        #...#\n\
        #####\n\
        .....\n\
        .....\
        ";
        assert_eq!(crate::part2(input), output);
    }
}
