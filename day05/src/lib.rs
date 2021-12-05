use std::collections::HashMap;

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
    type Point = (u32, u32);
    type Segment = (Point, Point);

    let segments = instructions
        .iter()
        .map(|line| {
            let points = line.split(" -> ")
                .map(|coordinate| {
                    let coordinates = coordinate.split(',')
                        .map(|part| part.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>();
                    return (coordinates[0], coordinates[1])
                })
                .collect::<Vec<Point>>();
            return (points[0], points[1])
        })
        .collect::<Vec<Segment>>();

    let mut memory = HashMap::<Point, u32>::new();

    for segment in segments {
        let x1 = segment.0.0;
        let y1 = segment.0.1;
        let x2 = segment.1.0;
        let y2 = segment.1.1;

        if x1 == x2 {
            let y_range = if y1 < y2 { y1..=y2 } else { y2..=y1 };
            for y in y_range {
                *memory.entry((x1, y)).or_insert(0) += 1;
            }
        } else if y1 == y2 {
            let x_range = if x1 < x2 { x1..=x2 } else { x2..=x1 };
            for x in x_range {
                *memory.entry((x, y1)).or_insert(0) += 1;
            }
        }
    }
    memory.iter().filter(|&(_, count)| *count >= 2).count()
}

pub fn part2(instructions: &[String]) -> u32 {
    0
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
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
        let sample_input: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let sample_output = 1924;
        assert_eq!(crate::part2(&sample_input), sample_output);
    }
}
