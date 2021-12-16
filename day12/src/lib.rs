use std::collections::{HashMap, HashSet};

pub fn part1(input: &str) -> usize {
    let mut multi_map: HashMap<&str, HashSet<&str>> = HashMap::new();

    input
        .lines()
        .map(|line| {
            let mut split = line.split('-');
            (split.next().unwrap(), split.next().unwrap())
        })
        .for_each(|(start, end)| {
            multi_map.entry(start).or_default().insert(end);
            multi_map.entry(end).or_default().insert(start);
        });

    fn seek(start: &str, multi_map: &HashMap<&str, HashSet<&str>>, small_caves_visited: &HashSet<&str>) -> usize {
        let mut count = 0;
        for &destination in multi_map.get(start).unwrap_or(&HashSet::new()) {
            if destination == "end" {
                count += 1;
            } else if destination == "start" {
                //
            } else if destination.chars().all(|c| c.is_lowercase()) {
                if !small_caves_visited.contains(destination) {
                    let mut small_caves_visited = small_caves_visited.clone();
                    small_caves_visited.insert(destination);
                    count += seek(destination, multi_map, &small_caves_visited);
                }
            } else {
                count += seek(destination, multi_map, small_caves_visited);
            }
        }
        count
    }

    seek("start", &multi_map, &HashSet::new())
}

pub fn part2(input: &str) -> usize {
    let mut multi_map: HashMap<&str, HashSet<&str>> = HashMap::new();

    input
        .lines()
        .map(|line| {
            let mut split = line.split('-');
            (split.next().unwrap(), split.next().unwrap())
        })
        .for_each(|(start, end)| {
            multi_map.entry(start).or_default().insert(end);
            multi_map.entry(end).or_default().insert(start);
        });

    fn seek(
        start: &str,
        multi_map: &HashMap<&str, HashSet<&str>>,
        small_caves_visited: &HashSet<&str>,
        used_double_visit: bool,
    ) -> usize {
        let mut count = 0;
        for &destination in multi_map.get(start).unwrap_or(&HashSet::new()) {
            if destination == "end" {
                count += 1;
            } else if destination == "start" {
                //
            } else if destination.chars().all(|c| c.is_lowercase()) {
                if !small_caves_visited.contains(destination) {
                    let mut small_caves_visited = small_caves_visited.clone();
                    small_caves_visited.insert(destination);
                    count += seek(destination, multi_map, &small_caves_visited, used_double_visit);
                } else if !used_double_visit {
                    count += seek(destination, multi_map, &small_caves_visited, true);
                }
            } else {
                count += seek(destination, multi_map, small_caves_visited, used_double_visit);
            }
        }
        count
    }

    seek("start", &multi_map, &HashSet::new(), false)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1_a() {
        let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        assert_eq!(crate::part1(input), 10);
    }

    #[test]
    fn test_part1_b() {
        let input = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
        assert_eq!(crate::part1(input), 19);
    }

    #[test]
    fn test_part1_c() {
        let input = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
        assert_eq!(crate::part1(input), 226);
    }

    #[test]
    fn test_part2_a() {
        let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        assert_eq!(crate::part2(input), 36);
    }

    #[test]
    fn test_part2_b() {
        let input = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
        assert_eq!(crate::part2(input), 103);
    }

    #[test]
    fn test_part2_c() {
        let input = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
        assert_eq!(crate::part2(input), 3509);
    }
}
