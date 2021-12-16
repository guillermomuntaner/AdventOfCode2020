use array2d::Array2D;

pub fn part1(input: &str) -> usize {
    let lines = input
        .lines()
        .map(|instruction| {
            instruction
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut map = Array2D::<u32>::from_rows(&lines);

    let neighbours: Vec<(i32, i32)> = vec![(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];

    fn flash(map: &mut Array2D<u32>, point: (usize, usize), neighbours: &Vec<(i32, i32)>, count: &mut usize) {
        *count += 1;
        for neighbour in neighbours {
            let adjacent_x = point.1 as i32 + neighbour.0;
            let adjacent_y = point.0 as i32 + neighbour.1;

            let is_within =
                (0..map.num_columns() as i32).contains(&adjacent_x) && (0..map.num_rows() as i32).contains(&adjacent_y);
            if !is_within {
                continue;
            }

            let adjacent_point = (adjacent_y as usize, adjacent_x as usize);

            map[adjacent_point] += 1;
            if map[adjacent_point] == 10 {
                flash(map, adjacent_point, neighbours, count)
            }
        }
    }

    let mut count = 0;
    for _ in 0..100 {
        for y in 0..map.num_rows() {
            for x in 0..map.num_columns() {
                let point = (y, x);
                map[point] += 1;
                if map[point] == 10 {
                    flash(&mut map, point, &neighbours, &mut count);
                }
            }
        }

        for y in 0..map.num_rows() {
            for x in 0..map.num_columns() {
                let point = (y, x);
                if map[point] >= 10 {
                    map[point] = 0;
                }
            }
        }
    }

    count
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

    let mut map = Array2D::<u32>::from_rows(&lines);

    let neighbours: Vec<(i32, i32)> = vec![(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];

    fn flash(map: &mut Array2D<u32>, point: (usize, usize), neighbours: &Vec<(i32, i32)>) {
        for neighbour in neighbours {
            let adjacent_x = point.1 as i32 + neighbour.0;
            let adjacent_y = point.0 as i32 + neighbour.1;

            let is_within =
                (0..map.num_columns() as i32).contains(&adjacent_x) && (0..map.num_rows() as i32).contains(&adjacent_y);
            if !is_within {
                continue;
            }

            let adjacent_point = (adjacent_y as usize, adjacent_x as usize);

            map[adjacent_point] += 1;
            if map[adjacent_point] == 10 {
                flash(map, adjacent_point, neighbours)
            }
        }
    }

    let mut step = 1;
    loop {
        for y in 0..map.num_rows() {
            for x in 0..map.num_columns() {
                let point = (y, x);
                map[point] += 1;
                if map[point] == 10 {
                    flash(&mut map, point, &neighbours);
                }
            }
        }

        for y in 0..map.num_rows() {
            for x in 0..map.num_columns() {
                let point = (y, x);
                if map[point] >= 10 {
                    map[point] = 0;
                }
            }
        }

        if map.elements_column_major_iter().all(|n| *n == 0) {
            break;
        }

        step += 1
    }

    step
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        assert_eq!(crate::part1(input), 1656);
    }

    #[test]
    fn test_part2() {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        assert_eq!(crate::part2(input), 195);
    }
}
