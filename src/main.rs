// #![feature(test)]
// extern crate test;

fn main() {
    let day1_input = inpututils::read_all_as::<u32>("inputs/day01");
    println!("Day 1 - Part 1: {}", day01::part1(&day1_input));
    println!("Day 1 - Part 2: {}", day01::part2(&day1_input));

    let day2_input = inpututils::read_all("inputs/day02");
    println!("Day 2 - Part 1: {}", day02::part1(&day2_input));
    println!("Day 2 - Part 2: {}", day02::part2(&day2_input));

    let day3_input = inpututils::read_all("inputs/day03");
    println!("Day 3 - Part 1: {}", day03::part1(&day3_input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_day_01_part1() {
        let day1_input = inpututils::read_all_as::<u32>("inputs/day01");
        assert_eq!(day01::part1(&day1_input), 1482);
        assert_eq!(day01::part2(&day1_input), 1518);
    }

    //#[bench]
    //fn bench_day_01_part1(b: &mut Bencher) {
    //    let input = inpututils::read_all_as::<u32>("inputs/day01");
    //    b.iter(|| day01::part1(&input));
    //}

    #[test]
    fn test_day_01_part2() {
        let day1_input = inpututils::read_all_as::<u32>("inputs/day01");
        assert_eq!(day01::part2(&day1_input), 1518);
    }

    //#[bench]
    //fn bench_day_01_part2(b: &mut Bencher) {
    //    let input = inpututils::read_all_as::<u32>("inputs/day01");
    //    b.iter(|| day01::part2(&input));
    //}
//
    //#[bench]
    //fn bench_day_01_part2_func(b: &mut Bencher) {
    //    let input = inpututils::read_all_as::<u32>("inputs/day01");
    //    b.iter(|| day01::part2_functional(&input));
    //}

    #[test]
    fn test_day_02_part1() {
        let day3_input = inpututils::read_all("inputs/day03");
        assert_eq!(day03::part1(&day3_input), 1427868);
    }
}
