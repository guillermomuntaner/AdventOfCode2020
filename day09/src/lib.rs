use array2d::Array2D;

pub fn part1(instructions: &[String]) -> u32 {
    let lines = instructions
        .iter()
        .map(|instruction| {
            instruction.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let map = Array2D::<u32>::from_rows(&lines);

    let mut sum = 0;
    let adjacent_points: Vec<(i32, i32)> = vec![(-1,0), (1,0), (0, -1), (0,1)];
    for x in 0..map.num_columns() {
        'map_loop: for y in 0..map.num_rows() {
            if adjacent_points.iter()
                .map(|adjacent_point| (x as i32 + adjacent_point.0, y as i32 + adjacent_point.1))
                .all(|adjacent_coordinate| {
                    let is_within = (0..map.num_columns() as i32).contains(&adjacent_coordinate.0)
                        && (0..map.num_rows() as i32).contains(&adjacent_coordinate.1);
                    if is_within {
                        map[(y, x)] < map[(adjacent_coordinate.1 as usize, adjacent_coordinate.0 as usize)]
                    } else {
                        true
                    }
                }) {
                sum += map[(y, x)] + 1
            }
        }
    }
    return sum
}

pub fn part2(instructions: &[String]) -> usize {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";
        let sample_input: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let sample_output = 15;
        assert_eq!(crate::part1(&sample_input), sample_output);
    }

    #[test]
    fn test_part2() {
        let input = "";
        let sample_input: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let sample_output = 26;
        assert_eq!(crate::part2(&sample_input), sample_output);
    }
}
