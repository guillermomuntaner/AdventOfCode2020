pub fn part1(input: &str) -> i64 {
    launch_probe(input).0
}

pub fn part2(input: &str) -> usize {
    launch_probe(input).1
}

fn launch_probe(input: &str) -> (i64, usize) {
    let (x_range, y_range) = input
        .strip_prefix("target area: x=")
        .unwrap()
        .split_once(", y=")
        .unwrap();
    let (min_x, max_x) = x_range.split_once("..").unwrap();
    let (min_y, max_y) = y_range.split_once("..").unwrap();
    let min_x = min_x.parse::<i64>().unwrap();
    let max_x = max_x.parse::<i64>().unwrap();
    let min_y = min_y.parse::<i64>().unwrap();
    let max_y = max_y.parse::<i64>().unwrap();

    // Note: Initially implemented with brute force; the altenative would be calculating all values using inequalities
    // ix = 0
    // iy = 0
    // ivx = ?
    // ivy = ?
    // ax = -1/s
    // ay = -1/s
    // x(t) = 0 + ivx * t - 1/2 * t^2
    // y(t) = 0 + ivy * t - 1/2 * t^2

    let mut successful_max_height: Vec<i64> = Vec::new();

    for ivx in 1..1000 {
        for ivy in -1000..1000 {
            let mut x = 0;
            let mut y = 0;
            let mut vx = ivx;
            let mut vy = ivy;
            let mut highest_y = y;
            for _ in 0..1000 {
                x += vx;
                y += vy;
                vx = if vx > 0 {
                    vx - 1
                } else if vx >= 0 {
                    0
                } else {
                    vx + 1
                };
                vy -= 1;

                if y > highest_y {
                    highest_y = y;
                }

                if x >= min_x && y <= max_y {
                    if x <= max_x && y >= min_y {
                        successful_max_height.push(highest_y);
                    }
                    break;
                }
            }
        }
    }

    (
        *successful_max_height.iter().max().unwrap(),
        successful_max_height.len(),
    )
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let input = "target area: x=20..30, y=-10..-5";
        assert_eq!(crate::part1(input), 45);
    }

    #[test]
    fn test_part1_failed_attempts() {
        let input = "target area: x=175..227, y=-134..-79";
        assert_ne!(crate::part1(input), 1176);
        assert_ne!(crate::part1(input), 4950);
    }

    #[test]
    fn test_part2() {
        let input = "target area: x=20..30, y=-10..-5";
        assert_eq!(crate::part2(input), 112);
    }

    #[test]
    fn test_part2_failed_attempts() {
        let input = "target area: x=175..227, y=-134..-79";
        assert_ne!(crate::part2(input), 1360);
    }
}
