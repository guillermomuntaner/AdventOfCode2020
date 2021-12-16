use array2d::Array2D;
use std::collections::HashSet;

pub fn part1(input: &str) -> u32 {
    let lines = input
        .lines()
        .map(|instruction| {
            instruction
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let map = Array2D::<u32>::from_rows(&lines);

    let mut sum = 0;
    let adjacent_points: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    for x in 0..map.num_columns() {
        for y in 0..map.num_rows() {
            if adjacent_points
                .iter()
                .map(|adjacent_point| (x as i32 + adjacent_point.0, y as i32 + adjacent_point.1))
                .all(|adjacent_coordinate| {
                    let is_within = (0..map.num_columns() as i32).contains(&adjacent_coordinate.0)
                        && (0..map.num_rows() as i32).contains(&adjacent_coordinate.1);
                    if is_within {
                        map[(y, x)] < map[(adjacent_coordinate.1 as usize, adjacent_coordinate.0 as usize)]
                    } else {
                        true
                    }
                })
            {
                sum += map[(y, x)] + 1
            }
        }
    }
    return sum;
}

pub fn part2(input: &str) -> usize {
    let lines = input
        .lines()
        .map(|instruction| {
            instruction
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let map = Array2D::<u32>::from_rows(&lines);

    let mut known_basin_points: HashSet<(usize, usize)> = HashSet::new();

    fn check_for_basin(
        x: usize,
        y: usize,
        map: &Array2D<u32>,
        known_basin_points: &mut HashSet<(usize, usize)>,
    ) -> usize {
        let adjacent_points: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        if known_basin_points.contains(&(x, y)) || map[(y, x)] == 9 {
            return 0;
        }
        known_basin_points.insert((x, y));
        return 1 + adjacent_points
            .iter()
            .map(|adjacent_point| (x as i32 + adjacent_point.0, y as i32 + adjacent_point.1))
            .filter(|adjacent_coordinate| {
                (0..map.num_columns() as i32).contains(&adjacent_coordinate.0)
                    && (0..map.num_rows() as i32).contains(&adjacent_coordinate.1)
            })
            .map(|adjacent_coordinate| (adjacent_coordinate.0 as usize, adjacent_coordinate.1 as usize))
            .map(|adjacent_coordinate| {
                check_for_basin(adjacent_coordinate.0, adjacent_coordinate.1, map, known_basin_points)
            })
            .sum::<usize>();
    }

    let mut known_basin_sizes: Vec<usize> = Vec::new();

    for x in 0..map.num_columns() {
        for y in 0..map.num_rows() {
            let basin_size = check_for_basin(x, y, &map, &mut known_basin_points);
            if basin_size > 0 {
                known_basin_sizes.push(basin_size)
            }
        }
    }

    known_basin_sizes.sort_by(|a, b| b.cmp(a));

    return known_basin_sizes
        .iter()
        .take(3)
        .fold(1, |acc, basin_size| acc * basin_size);
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
        assert_eq!(crate::part1(input), 15);
    }

    #[test]
    fn test_part2() {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";
        assert_eq!(crate::part2(input), 1134);
    }
}
