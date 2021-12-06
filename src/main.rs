fn main() {
    let day1_input = inpututils::read_all_as::<u32>("inputs/day01");
    println!("Day 1 - Part 1: {}", day01::part1(&day1_input));
    println!("Day 1 - Part 2: {}", day01::part2(&day1_input));

    let day2_input = inpututils::read_all("inputs/day02");
    println!("Day 2 - Part 1: {}", day02::part1(&day2_input));
    println!("Day 2 - Part 2: {}", day02::part2(&day2_input));

    let day3_input = inpututils::read_all("inputs/day03");
    println!("Day 3 - Part 1: {}", day03::part1(&day3_input));
    println!("Day 3 - Part 2: {}", day03::part2(&day3_input));

    let day4_input = inpututils::read_all("inputs/day04");
    println!("Day 4 - Part 1: {}", day04::part1(&day4_input));
    println!("Day 4 - Part 1: {}", day04::part2(&day4_input));

    let day5_input = inpututils::read_all("inputs/day05");
    println!("Day 5 - Part 1: {}", day05::part1(&day5_input));
    println!("Day 5 - Part 1: {}", day05::part2(&day5_input));

    let day6_input = std::fs::read_to_string("inputs/day06")
        .unwrap_or_else(|_| panic!("file not found: {}", "inputs/day06"))
        .split(',')
        .map(|number| number.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();
    println!("Day 6 - Part 1: {}", day06::part1(&day6_input));
    println!("Day 6 - Part 1: {}", day06::part2(&day6_input));
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_day_01() {
        let day1_input = inpututils::read_all_as::<u32>("inputs/day01");
        assert_eq!(day01::part1(&day1_input), 1482);
        assert_eq!(day01::part2(&day1_input), 1518);
    }

    #[test]
    fn test_day_02() {
        let day2_input = inpututils::read_all("inputs/day02");
        assert_eq!(day02::part1(&day2_input), 1427868);
        assert_eq!(day02::part2(&day2_input), 1568138742);
    }

    #[test]
    fn test_day_03() {
        let day3_input = inpututils::read_all("inputs/day03");
        assert_eq!(day03::part1(&day3_input), 3895776);
        assert_eq!(day03::part2(&day3_input), 7928162);
    }

    #[test]
    fn test_day_04() {
        let input = inpututils::read_all("inputs/day04");
        assert_eq!(day04::part1(&input), 8442);
        assert_eq!(day04::part2(&input), 4590);
    }

    #[test]
    fn test_day_05() {
        let input = inpututils::read_all("inputs/day05");
        assert_eq!(day05::part1(&input), 7436);
        assert_eq!(day05::part2(&input), 21104);
    }

    #[test]
    fn test_day_06() {
        let input = inpututils::read_all("inputs/day06");
        assert_eq!(day06::part1(&input), 0);
        assert_eq!(day06::part2(&input), 0);
    }
}
