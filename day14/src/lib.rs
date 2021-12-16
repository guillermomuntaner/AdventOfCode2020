use itertools::{Itertools, MinMaxResult};
use std::collections::HashMap;

pub fn part1(input: &str) -> usize {
    evolve(input, 10)
}

pub fn part2(input: &str) -> usize {
    evolve(input, 40)
}

fn evolve(input: &str, iterations: usize) -> usize {
    let (template_str, rules_str) = input.split_once("\n\n").unwrap();

    let template = template_str.chars().tuple_windows().counts();

    let rules: HashMap<(char, char), char> = rules_str
        .lines()
        .filter_map(|l| l.split(" -> ").collect_tuple::<(&str, &str)>())
        .map(|(lhs, rhs)| {
            let mut chars = lhs.chars();
            (
                (chars.next().unwrap(), chars.next().unwrap()),
                rhs.chars().next().unwrap(),
            )
        })
        .collect();

    fn grow(
        polymer: HashMap<(char, char), usize>,
        rules: &HashMap<(char, char), char>,
    ) -> HashMap<(char, char), usize> {
        let mut output: HashMap<(char, char), usize> = HashMap::new();
        for ((e1, e2), count) in polymer {
            if let Some(&new_e) = rules.get(&(e1, e2)) {
                *output.entry((e1, new_e)).or_insert(0) += count;
                *output.entry((new_e, e2)).or_insert(0) += count;
            } else {
                *output.entry((e1, e2)).or_insert(0) += count;
            }
        }
        output
    }

    let polymer = (0..iterations).fold(template, |polymer, _| grow(polymer, &rules));

    let mut histogram = HashMap::new();
    // The pairs are overlapping, so only count the 1st char of each pair and add the final char.
    for ((e1, _), count) in polymer {
        *histogram.entry(e1).or_insert(0) += count;
    }
    *histogram.entry(template_str.chars().last().unwrap()).or_insert(0) += 1;

    match histogram.values().minmax() {
        MinMaxResult::MinMax(&min, &max) => max - min,
        _ => panic!("Min max not found"),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
        assert_eq!(crate::part1(input), 1588);
    }

    #[test]
    fn test_part2() {
        let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
        assert_eq!(crate::part2(input), 2188189693529);
    }
}
