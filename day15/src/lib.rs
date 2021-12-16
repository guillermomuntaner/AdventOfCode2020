use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

fn dijkstra(start: Vertex, adjacency_list: &HashMap<Vertex, Vec<(Vertex, usize)>>) -> HashMap<Vertex, usize> {
    let mut distances = HashMap::new();
    let mut visited = HashSet::new();
    let mut to_visit = BinaryHeap::new();

    distances.insert(start, 0);
    to_visit.push(Visit {
        vertex: start,
        distance: 0,
    });

    while let Some(Visit { vertex, distance }) = to_visit.pop() {
        if !visited.insert(vertex) {
            // Already visited this node
            continue;
        }

        if let Some(neighbors) = adjacency_list.get(&vertex) {
            for (neighbor, cost) in neighbors {
                let new_distance = distance + cost;
                let is_shorter = distances.get(&neighbor).map_or(true, |&current| new_distance < current);

                if is_shorter {
                    distances.insert(*neighbor, new_distance);
                    to_visit.push(Visit {
                        vertex: *neighbor,
                        distance: new_distance,
                    });
                }
            }
        }
    }

    distances
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Vertex {
    x: usize,
    y: usize,
}

impl Vertex {
    fn new(x: usize, y: usize) -> Vertex {
        Vertex { x, y }
    }
}

#[derive(Debug)]
struct Visit<V> {
    vertex: V,
    distance: usize,
}

impl<V> Ord for Visit<V> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl<V> PartialOrd for Visit<V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<V> PartialEq for Visit<V> {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl<V> Eq for Visit<V> {}

//

pub fn part1(input: &str) -> usize {
    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .enumerate()
                .map(|(x, weight)| (Vertex::new(x, y), weight))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut adjacency_list = HashMap::new();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let mut neightbours: Vec<(Vertex, usize)> = Vec::new();
            if y > 0 {
                neightbours.push(map[y - 1][x]);
            }
            if y < map.len() - 1 {
                neightbours.push(map[y + 1][x]);
            }
            if x > 0 {
                neightbours.push(map[y][x - 1]);
            }
            if x < map[y].len() - 1 {
                neightbours.push(map[y][x + 1]);
            }
            adjacency_list.insert(map[y][x].0, neightbours);
        }
    }

    let distances = dijkstra(map[0][0].0, &adjacency_list);

    *distances.get(&map.last().unwrap().last().unwrap().0).unwrap()
}

pub fn part2(input: &str) -> usize {
    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .enumerate()
                .map(|(x, weight)| (Vertex::new(x, y), weight))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut enlarged_map: Vec<Vec<(Vertex, usize)>> = Vec::new();

    let col_len = map.len();
    for i in 0..5 {
        for y in 0..col_len {
            let row = &map[y];
            let row_len = row.len();

            let mut enlarged_row: Vec<(Vertex, usize)> = Vec::new();

            for j in 0..5 {
                for x in 0..row_len {
                    let mut risk = row[x].1 + j + i;
                    while risk > 9 {
                        risk -= 9
                    }
                    enlarged_row.push((Vertex::new(i * col_len + y, j * row_len + x), risk))
                }
            }
            enlarged_map.push(enlarged_row)
        }
    }

    let mut adjacency_list = HashMap::new();
    for y in 0..enlarged_map.len() {
        for x in 0..enlarged_map[y].len() {
            let mut neightbours: Vec<(Vertex, usize)> = Vec::new();
            if y > 0 {
                neightbours.push(enlarged_map[y - 1][x]);
            }
            if y < enlarged_map.len() - 1 {
                neightbours.push(enlarged_map[y + 1][x]);
            }
            if x > 0 {
                neightbours.push(enlarged_map[y][x - 1]);
            }
            if x < enlarged_map[y].len() - 1 {
                neightbours.push(enlarged_map[y][x + 1]);
            }
            adjacency_list.insert(enlarged_map[y][x].0, neightbours);
        }
    }

    let distances = dijkstra(enlarged_map[0][0].0, &adjacency_list);

    *distances.get(&enlarged_map.last().unwrap().last().unwrap().0).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
        assert_eq!(crate::part1(input), 40);
    }

    #[test]
    fn test_part2() {
        let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
        assert_eq!(crate::part2(input), 315);
    }
}
