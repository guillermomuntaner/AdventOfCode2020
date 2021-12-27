fn main() {
    println!("Day 1 - Part 1: {}", day01::part1(include_str!("../inputs/day01")));
    println!("Day 1 - Part 2: {}", day01::part2(include_str!("../inputs/day01")));

    println!("Day 2 - Part 1: {}", day02::part1(include_str!("../inputs/day02")));
    println!("Day 2 - Part 2: {}", day02::part2(include_str!("../inputs/day02")));

    let day3_input = inpututils::read_lines("inputs/day03");
    println!("Day 3 - Part 1: {}", day03::part1(&day3_input));
    println!("Day 3 - Part 2: {}", day03::part2(&day3_input));

    let day4_input = inpututils::read_lines("inputs/day04");
    println!("Day 4 - Part 1: {}", day04::part1(&day4_input));
    println!("Day 4 - Part 2: {}", day04::part2(&day4_input));

    let day5_input = inpututils::read_lines("inputs/day05");
    println!("Day 5 - Part 1: {}", day05::part1(&day5_input));
    println!("Day 5 - Part 2: {}", day05::part2(&day5_input));

    let day6_input = inpututils::read_comma_separated_as::<u8>("inputs/day06");
    println!("Day 6 - Part 1: {}", day06::part1(&day6_input));
    println!("Day 6 - Part 2: {}", day06::part2(&day6_input));

    let day7_input = inpututils::read_comma_separated_as::<u64>("inputs/day07");
    let mut day7_pt1_input = day7_input.to_vec();
    println!("Day 7 - Part 1: {}", day07::part1(&mut day7_pt1_input));
    println!("Day 7 - Part 2: {}", day07::part2(&day7_input));

    println!("Day 8 - Part 1: {}", day08::part1(include_str!("../inputs/day08")));
    println!("Day 8 - Part 2: {}", day08::part2(include_str!("../inputs/day08")));

    println!("Day 9 - Part 1: {}", day09::part1(include_str!("../inputs/day09")));
    println!("Day 9 - Part 2: {}", day09::part2(include_str!("../inputs/day09")));

    println!("Day 10 - Part 1: {}", day10::part1(include_str!("../inputs/day10")));
    println!("Day 10 - Part 2: {}", day10::part2(include_str!("../inputs/day10")));

    println!("Day 11 - Part 1: {}", day11::part1(include_str!("../inputs/day11")));
    println!("Day 11 - Part 2: {}", day11::part2(include_str!("../inputs/day11")));

    println!("Day 12 - Part 1: {}", day12::part1(include_str!("../inputs/day12")));
    println!("Day 12 - Part 2: {}", day12::part2(include_str!("../inputs/day12")));

    println!("Day 13 - Part 1: {}", day13::part1(include_str!("../inputs/day13")));
    println!("Day 13 - Part 2: {}", day13::part2(include_str!("../inputs/day13")));

    println!("Day 14 - Part 1: {}", day14::part1(include_str!("../inputs/day14")));
    println!("Day 14 - Part 2: {}", day14::part2(include_str!("../inputs/day14")));

    println!("Day 15 - Part 1: {}", day15::part1(include_str!("../inputs/day15")));
    println!("Day 15 - Part 2: {}", day15::part2(include_str!("../inputs/day15")));

    println!("Day 16 - Part 1: {}", day16::part1(include_str!("../inputs/day16")));
    println!("Day 16 - Part 2: {}", day16::part2(include_str!("../inputs/day16")));

    println!("Day 17 - Part 1: {}", day17::part1(include_str!("../inputs/day17")));
    println!("Day 17 - Part 2: {}", day17::part2(include_str!("../inputs/day17")));

    println!("Day 18 - Part 1: {}", day18::part1(include_str!("../inputs/day18")));
    println!("Day 18 - Part 2: {}", day18::part2(include_str!("../inputs/day18")));
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_day_01() {
        assert_eq!(day01::part1(include_str!("../inputs/day01")), 1482);
        assert_eq!(day01::part2(include_str!("../inputs/day01")), 1518);
    }

    #[test]
    fn test_day_02() {
        assert_eq!(day02::part1(include_str!("../inputs/day02")), 1427868);
        assert_eq!(day02::part2(include_str!("../inputs/day02")), 1568138742);
    }

    #[test]
    fn test_day_03() {
        let day3_input = inpututils::read_lines("inputs/day03");
        assert_eq!(day03::part1(&day3_input), 3895776);
        assert_eq!(day03::part2(&day3_input), 7928162);
    }

    #[test]
    fn test_day_04() {
        let input = inpututils::read_lines("inputs/day04");
        assert_eq!(day04::part1(&input), 8442);
        assert_eq!(day04::part2(&input), 4590);
    }

    #[test]
    fn test_day_05() {
        let input = inpututils::read_lines("inputs/day05");
        assert_eq!(day05::part1(&input), 7436);
        assert_eq!(day05::part2(&input), 21104);
    }

    #[test]
    fn test_day_06() {
        let input = inpututils::read_comma_separated_as::<u8>("inputs/day06");
        assert_eq!(day06::part1(&input), 386640);
        assert_eq!(day06::part2(&input), 1733403626279);
    }

    #[test]
    fn test_day_07() {
        let input = inpututils::read_comma_separated_as::<u64>("inputs/day07");
        let mut pt1_input = input.to_vec();
        assert_eq!(day07::part1(&mut pt1_input), 352997);
        assert_eq!(day07::part2(&input), 101571302);
    }

    #[test]
    fn test_day_08() {
        assert_eq!(day08::part1(include_str!("../inputs/day08")), 274);
        assert_eq!(day08::part2(include_str!("../inputs/day08")), 1012089);
    }

    #[test]
    fn test_day_09() {
        assert_eq!(day09::part1(include_str!("../inputs/day09")), 465);
        assert_eq!(day09::part2(include_str!("../inputs/day09")), 1269555);
    }

    #[test]
    fn test_day_10() {
        assert_eq!(day10::part1(include_str!("../inputs/day10")), 168417);
        assert_eq!(day10::part2(include_str!("../inputs/day10")), 2802519786);
    }

    #[test]
    fn test_day_11() {
        assert_eq!(day11::part1(include_str!("../inputs/day11")), 1594);
        assert_eq!(day11::part2(include_str!("../inputs/day11")), 437);
    }

    #[test]
    fn test_day_12() {
        assert_eq!(day12::part1(include_str!("../inputs/day12")), 4011);
        assert_eq!(day12::part2(include_str!("../inputs/day12")), 108035);
    }

    #[test]
    fn test_day_13() {
        assert_eq!(day13::part1(include_str!("../inputs/day13")), 729);
        assert_eq!(
            day13::part2(include_str!("../inputs/day13")),
            "\
        ###...##..####.#....###..#..#.####.###..\n\
        #..#.#..#....#.#....#..#.#..#.#....#..#.\n\
        #..#.#......#..#....###..####.###..#..#.\n\
        ###..#.##..#...#....#..#.#..#.#....###..\n\
        #.#..#..#.#....#....#..#.#..#.#....#....\n\
        #..#..###.####.####.###..#..#.#....#....\
        "
        );
    }

    #[test]
    fn test_day_14() {
        assert_eq!(day14::part1(include_str!("../inputs/day14")), 2602);
        assert_eq!(day14::part2(include_str!("../inputs/day14")), 2942885922173);
    }

    #[test]
    fn test_day_15() {
        assert_eq!(day15::part1(include_str!("../inputs/day15")), 698);
        assert_eq!(day15::part2(include_str!("../inputs/day15")), 3022);
    }

    #[test]
    fn test_day_16() {
        assert_eq!(day16::part1(include_str!("../inputs/day16")), 1038);
        assert_eq!(day16::part2(include_str!("../inputs/day16")), 246761930504);
    }

    #[test]
    fn test_day_17() {
        assert_eq!(day17::part1(include_str!("../inputs/day17")), 8911);
        assert_eq!(day17::part2(include_str!("../inputs/day17")), 4748);
    }

    #[test]
    fn test_day_18() {
        assert_eq!(day18::part1(include_str!("../inputs/day18")), 3734);
        assert_eq!(day18::part2(include_str!("../inputs/day18")), 4837);
    }
}
