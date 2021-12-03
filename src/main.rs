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
}
