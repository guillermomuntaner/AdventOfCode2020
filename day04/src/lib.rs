use array2d::Array2D;

/// --- Day 4: Giant Squid ---
/// You're already almost 1.5km (almost a mile) below the surface of the ocean, already so deep that you can't see any sunlight. What you can see, however, is a giant squid that has attached itself to the outside of your submarine.
///
/// Maybe it wants to play bingo?
///
/// Bingo is played on a set of boards each consisting of a 5x5 grid of numbers. Numbers are chosen at random, and the chosen number is marked on all boards on which it appears. (Numbers may not appear on all boards.) If all numbers in any row or any column of a board are marked, that board wins. (Diagonals don't count.)
///
/// The submarine has a bingo subsystem to help passengers (currently, you and the giant squid) pass the time. It automatically generates a random order in which to draw numbers and a random set of boards (your puzzle input). For example:
///
/// 7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1
///
/// 22 13 17 11  0
/// 8  2 23  4 24
/// 21  9 14 16  7
/// 6 10  3 18  5
/// 1 12 20 15 19
///
/// 3 15  0  2 22
/// 9 18 13 17  5
/// 19  8  7 25 23
/// 20 11 10 24  4
/// 14 21 16 12  6
///
/// 14 21 17 24  4
/// 10 16 15  9 19
/// 18  8 23 26 20
/// 22 11 13  6  5
/// 2  0 12  3  7
/// After the first five numbers are drawn (7, 4, 9, 5, and 11), there are no winners, but the boards are marked as follows (shown here adjacent to each other to save space):
///
/// 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
/// 8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
/// 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
/// 6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
/// 1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
/// After the next six numbers are drawn (17, 23, 2, 0, 14, and 21), there are still no winners:
///
/// 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
/// 8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
/// 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
/// 6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
/// 1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
/// Finally, 24 is drawn:
///
/// 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
/// 8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
/// 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
/// 6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
/// 1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
/// At this point, the third board wins because it has at least one complete row or column of marked numbers (in this case, the entire top row is marked: 14 21 17 24 4).
///
/// The score of the winning board can now be calculated. Start by finding the sum of all unmarked numbers on that board; in this case, the sum is 188. Then, multiply that sum by the number that was just called when the board won, 24, to get the final score, 188 * 24 = 4512.
///
/// To guarantee victory against the giant squid, figure out which board will win first. What will your final score be if you choose that board?
/// --- Day 2: Dive! ---
/// Now, you need to figure out how to pilot this thing.
///
/// It seems like the submarine can take a series of commands like forward 1, down 2, or up 3:
///
/// forward X increases the horizontal position by X units.
/// down X increases the depth by X units.
/// up X decreases the depth by X units.
/// Note that since you're on a submarine, down and up affect your depth, and so they have the opposite result of what you might expect.
///
/// The submarine seems to already have a planned course (your puzzle input). You should probably figure out where it's going. For example:
///
/// forward 5
/// down 5
/// forward 8
/// up 3
/// down 8
/// forward 2
/// Your horizontal position and depth both start at 0. The steps above would then modify them as follows:
///
/// forward 5 adds 5 to your horizontal position, a total of 5.
/// down 5 adds 5 to your depth, resulting in a value of 5.
/// forward 8 adds 8 to your horizontal position, a total of 13.
/// up 3 decreases your depth by 3, resulting in a value of 2.
/// down 8 adds 8 to your depth, resulting in a value of 10.
/// forward 2 adds 2 to your horizontal position, a total of 15.
/// After following these instructions, you would have a horizontal position of 15 and a depth of 10. (Multiplying these together produces 150.)
///
/// Calculate the horizontal position and depth you would have after following the planned course. What do you get if you multiply your final horizontal position by your final depth?
pub fn part1(instructions: &[String]) -> u32 {
    let mut iter = instructions.iter();

    let bingo_numbers = iter.next().unwrap()
        .split(',')
        .map(|number| number.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    iter.next().unwrap();

    #[derive(PartialEq, Debug, Clone)]
    struct BingoNumber {
        number: u32,
        isMarked: bool,
    }

    let mut bingo_boards: Vec<Array2D<BingoNumber>> = iter
        .collect::<Vec<&String>>()
        .split(|line| line.is_empty())
        .map(|board_lines| board_lines
            .iter()
            .map(|board_line| board_line
                .split_whitespace()
                .map(|number| BingoNumber {
                    number: number.parse::<u32>().unwrap(),
                    isMarked: false
                })
                .collect::<Vec<BingoNumber>>()
            )
            .collect::<Vec<Vec<BingoNumber>>>()
        )
        .map(|boards| Array2D::<BingoNumber>::from_rows(&boards))
        .collect();

    for number in bingo_numbers {
        'board_loop: for i in 0..bingo_boards.len() {
            for x in 0..bingo_boards[i].num_columns() {
                for y in 0..bingo_boards[i].num_rows() {
                    if bingo_boards[i][(y, x)].number == number {
                        bingo_boards[i][(y, x)].isMarked = true;
                        // Only check if this board become a winner. Only need to check current x & y.
                        if bingo_boards[i].column_iter(x).all(|bingoNumber| bingoNumber.isMarked)
                            || bingo_boards[i].row_iter(y).all(|bingoNumber| bingoNumber.isMarked) {
                            // Got a winner
                            let unmarked_sum: u32 = bingo_boards[i].elements_column_major_iter()
                                .filter(|bingo_number| bingo_number.isMarked == false)
                                .map(|bingo_number| bingo_number.number)
                                .sum();
                            return unmarked_sum * number
                        }
                        continue 'board_loop;
                    }
                }
            }
        }
    }

    panic!("No winner")
}

#[cfg(test)]
mod tests_part1 {
    #[test]
    fn test_part1() {
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
        let sample_output = 4512;
        assert_eq!(crate::part1(&sample_input), sample_output);
    }
}
