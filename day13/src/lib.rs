use array2d::Array2D;

pub fn part1(input: &str) -> usize {
    let mut map = Array2D::<bool>::filled_with(false, 3000, 3000);

    let (coordinates, instructions) = input.split_once("\n\n").unwrap();

    coordinates.lines()
        .map(|coordinate| coordinate.split(',').map(|c| c.parse::<usize>().unwrap()).collect::<Vec<_>>() )
        .map(|coordinate| (coordinate[1], coordinate[0]))
        .for_each(|coordinate| map[coordinate] = true);

    let final_map = instructions
        .lines()
        .take(1)
        .fold(map, |map, instruction| {
            let (command, position_str) = instruction.split_once('=').unwrap();
            let position = position_str.parse::<usize>().unwrap();
            match command {
                "fold along y" => {
                    let mut new_map = Array2D::<bool>::filled_with(false, position, map.num_columns());
                    for x in 0..new_map.num_columns() {
                        for y in 0..new_map.num_rows() {
                            if y < position {
                                if y <= 2*position && map[(2 * position - y, x)] {
                                    new_map[(y, x)] = true
                                } else {
                                    new_map[(y,x)] = map[(y,x)]
                                }
                            }
                        }
                    }
                    new_map
                }
                "fold along x" => {
                    let mut new_map = Array2D::<bool>::filled_with(false, map.num_rows(), position);
                    for x in 0..map.num_columns() {
                        for y in 0..map.num_rows() {
                            if x < position {
                                if x <= 2*position && map[(y, 2 * position - x)] {
                                    new_map[(y, x)] = true
                                } else {
                                    new_map[(y,x)] = map[(y,x)]
                                }
                            }
                        }
                    }
                    new_map
                }
                c => { panic!("Expected fold along x/y, got {}", c) }
            }
        });

    final_map.elements_column_major_iter().filter(|&e| *e).count()
}

pub fn part2(input: &str) -> String {
    let mut map = Array2D::<bool>::filled_with(false, 3000, 3000);

    let (coordinates, instructions) = input.split_once("\n\n").unwrap();

    coordinates.lines()
        .map(|coordinate| coordinate.split(',').map(|c| c.parse::<usize>().unwrap()).collect::<Vec<_>>() )
        .map(|coordinate| (coordinate[1], coordinate[0]))
        .for_each(|coordinate| map[coordinate] = true);

    let final_map = instructions
        .lines()
        .fold(map, |map, instruction| {
            let (command, position_str) = instruction.split_once('=').unwrap();
            let position = position_str.parse::<usize>().unwrap();
            match command {
                "fold along y" => {
                    let mut new_map = Array2D::<bool>::filled_with(false, position, map.num_columns());
                    for x in 0..new_map.num_columns() {
                        for y in 0..new_map.num_rows() {
                            if y < position {
                                if y <= 2*position && map[(2 * position - y, x)] {
                                    new_map[(y, x)] = true
                                } else {
                                    new_map[(y,x)] = map[(y,x)]
                                }
                            }
                        }
                    }
                    new_map
                }
                "fold along x" => {
                    let mut new_map = Array2D::<bool>::filled_with(false, map.num_rows(), position);
                    for x in 0..map.num_columns() {
                        for y in 0..map.num_rows() {
                            if x < position {
                                if x <= 2*position && map[(y, 2 * position - x)] {
                                    new_map[(y, x)] = true
                                } else {
                                    new_map[(y,x)] = map[(y,x)]
                                }
                            }
                        }
                    }
                    new_map
                }
                c => { panic!("Expected fold along x/y, got {}", c) }
            }
        });

    final_map.rows_iter()
        .map(|row| row.map(|c| if *c { '#' } else { '.' }).collect::<String>())
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
