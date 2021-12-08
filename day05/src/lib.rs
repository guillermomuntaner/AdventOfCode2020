use std::cmp::max;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

struct Segment {
    i: Point,
    f: Point,
}

/// --- Day 5: Hydrothermal Venture ---
/// You come across a field of hydrothermal vents on the ocean floor! These vents constantly produce large, opaque clouds, so it would be best to avoid them if possible.
///
/// They tend to form in lines; the submarine helpfully produces a list of nearby lines of vents (your puzzle input) for you to review. For example:
///
/// 0,9 -> 5,9
/// 8,0 -> 0,8
/// 9,4 -> 3,4
/// 2,2 -> 2,1
/// 7,0 -> 7,4
/// 6,4 -> 2,0
/// 0,9 -> 2,9
/// 3,4 -> 1,4
/// 0,0 -> 8,8
/// 5,5 -> 8,2
/// Each line of vents is given as a line segment in the format x1,y1 -> x2,y2 where x1,y1 are the coordinates of one end the line segment and x2,y2 are the coordinates of the other end. These line segments include the points at both ends. In other words:
///
/// An entry like 1,1 -> 1,3 covers points 1,1, 1,2, and 1,3.
/// An entry like 9,7 -> 7,7 covers points 9,7, 8,7, and 7,7.
/// For now, only consider horizontal and vertical lines: lines where either x1 = x2 or y1 = y2.
///
/// So, the horizontal and vertical lines from the above list would produce the following diagram:
///
/// .......1..
/// ..1....1..
/// ..1....1..
/// .......1..
/// .112111211
/// ..........
/// ..........
/// ..........
/// ..........
/// 222111....
/// In this diagram, the top left corner is 0,0 and the bottom right corner is 9,9. Each position is shown as the number of lines which cover that point or . if no line covers that point. The top-left pair of 1s, for example, comes from 2,2 -> 2,1; the very bottom row is formed by the overlapping lines 0,9 -> 5,9 and 0,9 -> 2,9.
///
/// To avoid the most dangerous areas, you need to determine the number of points where at least two lines overlap. In the above example, this is anywhere in the diagram with a 2 or larger - a total of 5 points.
///
/// Consider only horizontal and vertical lines. At how many points do at least two lines overlap?
pub fn part1(instructions: &[String]) -> usize {
    let segments = parse_segments(instructions);
    count_dangerous_areas(&segments, false)
}

/// --- Part Two ---
/// Unfortunately, considering only horizontal and vertical lines doesn't give you the full picture; you need to also consider diagonal lines.
///
/// Because of the limits of the hydrothermal vent mapping system, the lines in your list will only ever be horizontal, vertical, or a diagonal line at exactly 45 degrees. In other words:
///
/// An entry like 1,1 -> 3,3 covers points 1,1, 2,2, and 3,3.
/// An entry like 9,7 -> 7,9 covers points 9,7, 8,8, and 7,9.
/// Considering all lines from the above example would now produce the following diagram:
///
/// 1.1....11.
/// .111...2..
/// ..2.1.111.
/// ...1.2.2..
/// .112313211
/// ...1.2....
/// ..1...1...
/// .1.....1..
/// 1.......1.
/// 222111....
/// You still need to determine the number of points where at least two lines overlap. In the above example, this is still anywhere in the diagram with a 2 or larger - now a total of 12 points.
///
/// Consider all of the lines. At how many points do at least two lines overlap?
pub fn part2(instructions: &[String]) -> usize {
    let segments = parse_segments(instructions);
    count_dangerous_areas(&segments, true)
}

fn count_dangerous_areas(segments: &Vec<Segment>, count_diagonally: bool) -> usize {
    let mut memory = HashMap::<Point, i32>::new();

    for segment in segments {
        let x_distance = segment.f.x - segment.i.x;
        let x_direction = x_distance.signum();
        let width = x_distance * x_distance.signum();

        let y_distance = segment.f.y - segment.i.y;
        let y_direction = y_distance.signum();
        let height = y_distance * y_direction.signum();

        if width == 0 || height == 0 || (count_diagonally && width == height) {
            for offset in 0..=max(width, height) {
                let x = segment.i.x + offset * x_direction;
                let y = segment.i.y + offset * y_direction;
                *memory.entry(Point { x: x, y: y }).or_insert(0) += 1;
            }
        }
    }
    memory.iter().filter(|&(_, count)| *count >= 2).count()
}

fn parse_segments(instructions: &[String]) -> Vec<Segment> {
    instructions
        .iter()
        .map(|line| {
            let points = line
                .split(" -> ")
                .map(|coordinate| {
                    let coordinates = coordinate
                        .split(',')
                        .map(|part| part.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();
                    Point {
                        x: coordinates[0],
                        y: coordinates[1],
                    }
                })
                .collect::<Vec<Point>>();
            Segment {
                i: points[0],
                f: points[1],
            }
        })
        .collect::<Vec<Segment>>()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        let sample_input: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let sample_output = 5;
        assert_eq!(crate::part1(&sample_input), sample_output);
    }

    #[test]
    fn test_part2() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        let sample_input: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let sample_output = 12;
        assert_eq!(crate::part2(&sample_input), sample_output);
    }
}
