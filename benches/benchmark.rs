use criterion::{criterion_group, criterion_main, Criterion};

fn day1_benchmark(c: &mut Criterion) {
    let input = inpututils::read_lines_as::<u32>("inputs/day01");
    c.bench_function("Day 1 - Read input", |b| {
        b.iter(|| {
            inpututils::read_lines_as::<u32>("inputs/day01");
        })
    });
    c.bench_function("Day 1 - Part 1", |b| {
        b.iter(|| {
            let input = inpututils::read_lines_as::<u32>("inputs/day01");
            day01::part1(&input);
        })
    });
    c.bench_function("Day 1 - Part 1", |b| {
        b.iter(|| {
            let string = std::fs::read_to_string("inputs/day02").unwrap();
            day01::part1_iter(string.lines());
        })
    });
    c.bench_function("Day 1 - Part 2", |b| {
        b.iter(|| {
            day01::part2(&input);
        })
    });
    c.bench_function("Day 1 - Part 2 - Functional", |b| {
        b.iter(|| {
            day01::part2_functional(&input);
        })
    });
}

fn day2_benchmark(c: &mut Criterion) {
    let input = inpututils::read_lines("inputs/day02");
    c.bench_function("Day 2 - Read input", |b| {
        b.iter(|| {
            inpututils::read_lines("inputs/day02");
        })
    });
    c.bench_function("Day 2 - Part 1", |b| {
        b.iter(|| {
            let input = inpututils::read_lines("inputs/day02");
            day02::part1(&input);
        })
    });
    c.bench_function("Day 2 - Part 1 with regex", |b| {
        b.iter(|| {
            let input = inpututils::read_lines("inputs/day02");
            day02::part1_with_regex(&input);
        })
    });
    c.bench_function("Day 2 - Part 2 - Iter", |b| {
        b.iter(|| {
            let file = inpututils::read_file("inputs/day02");
            let lines_iter = inpututils::lines(&file);
            day02::part1_iter(lines_iter);
        })
    });
    c.bench_function("Day 2 - Part 2", |b| {
        b.iter(|| {
            day02::part2(&input);
        })
    });
}

fn day3_benchmark(c: &mut Criterion) {
    let input = inpututils::read_lines("inputs/day03");
    c.bench_function("Day 3 - Read input", |b| {
        b.iter(|| {
            inpututils::read_lines("inputs/day03");
        })
    });
    c.bench_function("Day 3 - Part 1", |b| {
        b.iter(|| {
            day03::part1(&input);
        })
    });
    c.bench_function("Day 3 - Part 2", |b| {
        b.iter(|| {
            day03::part2(&input);
        })
    });
}

fn day4_benchmark(c: &mut Criterion) {
    let input = inpututils::read_lines("inputs/day04");
    c.bench_function("Day 4 - Read input", |b| {
        b.iter(|| {
            inpututils::read_lines("inputs/day04");
        })
    });
    c.bench_function("Day 4 - Part 1", |b| {
        b.iter(|| {
            day04::part1(&input);
        })
    });
    c.bench_function("Day 4 - Part 2", |b| {
        b.iter(|| {
            day04::part2(&input);
        })
    });
}

fn day5_benchmark(c: &mut Criterion) {
    let input = inpututils::read_lines("inputs/day05");
    c.bench_function("Day 5- Read input", |b| {
        b.iter(|| {
            inpututils::read_lines("inputs/day05");
        })
    });
    c.bench_function("Day 5 - Part 1", |b| {
        b.iter(|| {
            day05::part1(&input);
        })
    });
    c.bench_function("Day 5 - Part 2", |b| {
        b.iter(|| {
            day05::part2(&input);
        })
    });
}

fn day6_benchmark(c: &mut Criterion) {
    let input = inpututils::read_comma_separated_as::<u8>("inputs/day06");
    c.bench_function("Day 6 - Read input", |b| {
        b.iter(|| {
            inpututils::read_comma_separated_as::<u8>("inputs/day06");
        })
    });
    c.bench_function("Day 6 - Part 1", |b| {
        b.iter(|| {
            day06::part1(&input);
        })
    });
    c.bench_function("Day 6 - Part 2", |b| {
        b.iter(|| {
            day06::part2(&input);
        })
    });
}

fn day7_benchmark(c: &mut Criterion) {
    let input = inpututils::read_comma_separated_as::<u64>("inputs/day07");
    c.bench_function("Day 7 - Read input", |b| {
        b.iter(|| {
            inpututils::read_comma_separated_as::<u64>("inputs/day07");
        })
    });
    let mut pt1_input = inpututils::read_comma_separated_as::<u64>("inputs/day07");
    c.bench_function("Day 7 - Part 1", |b| {
        b.iter(|| {
            day07::part1(&mut pt1_input);
        })
    });
    c.bench_function("Day 7 - Part 2", |b| {
        b.iter(|| {
            day07::part2(&input);
        })
    });
}

fn day8_benchmark(c: &mut Criterion) {
    let input = inpututils::read_lines("inputs/day08");
    c.bench_function("Day 8 - Read input", |b| {
        b.iter(|| {
            inpututils::read_lines("inputs/day08");
        })
    });
    c.bench_function("Day 8 - Part 1", |b| {
        b.iter(|| {
            day08::part1(&input);
        })
    });
    c.bench_function("Day 8 - Part 2", |b| {
        b.iter(|| {
            day08::part2(&input);
        })
    });
}

fn day9_benchmark(c: &mut Criterion) {
    let input = inpututils::read_lines("inputs/day09");
    c.bench_function("Day 9 - Read input", |b| {
        b.iter(|| {
            inpututils::read_lines("inputs/day09");
        })
    });
    c.bench_function("Day 9 - Part 1", |b| {
        b.iter(|| {
            day09::part1(&input);
        })
    });
    c.bench_function("Day 9 - Part 2", |b| {
        b.iter(|| {
            day09::part2(&input);
        })
    });
}

criterion_group!(
    benchmark,
    day1_benchmark,
    day2_benchmark,
    day3_benchmark,
    day4_benchmark,
    day5_benchmark,
    day6_benchmark,
    day7_benchmark,
    day8_benchmark,
    day8_benchmark,
    day9_benchmark
);
criterion_main!(benchmark);
