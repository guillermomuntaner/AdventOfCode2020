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
    let (numbers, mut boards) = parse_game(instructions);

    for number in numbers {
        for board in &mut boards {
            if scratch_number_and_call(number, board) {
                return number * count_non_scratched(board)
            }
        }
    }
    panic!("No winner")
}

/// --- Part Two ---
/// On the other hand, it might be wise to try a different strategy: let the giant squid win.
///
/// You aren't sure how many bingo boards a giant squid could play at once, so rather than waste time counting its arms, the safe thing to do is to figure out which board will win last and choose that one. That way, no matter which boards it picks, it will win for sure.
///
/// In the above example, the second board is the last to win, which happens after 13 is eventually called and its middle column is completely marked. If you were to keep playing until this point, the second board would have a sum of unmarked numbers equal to 148 for a final score of 148 * 13 = 1924.
///
/// Figure out which board will win last. Once it wins, what would its final score be?
pub fn part2(instructions: &[String]) -> u32 {
    let (numbers, mut boards) = parse_game(instructions);
    let mut board_has_won = vec![false; boards.len()];
    for number in numbers {
        for (i, board) in boards.iter_mut().enumerate() {
            if board_has_won[i] { continue }
            if scratch_number_and_call(number, board) {
                board_has_won[i] = true;
                if !board_has_won.contains(&false) {
                    return number * count_non_scratched(board)
                }
            }
        }
    }

    panic!("No winner")
}

fn parse_game(instructions: &[String]) -> (Vec<u32>, Vec<Array2D<Option<u32>>>){
    let lines = instructions[0]
        .split(',')
        .map(|number| number.parse::<u32>().unwrap())
        .collect();

    let board = instructions[2..instructions.len()]
        .split(|line| line.is_empty())
        .map(|board_lines| board_lines
            .iter()
            .map(|board_line| board_line
                .split_whitespace()
                .map(|number| number.parse::<u32>().unwrap())
                .map(|number| Some(number))
                .collect::<Vec<Option<u32>>>()
            )
            .collect::<Vec<Vec<Option<u32>>>>()
        )
        .map(|board| Array2D::<Option<u32>>::from_rows(&board))
        .collect();

    return (lines, board)
}

fn scratch_number_and_call(number: u32, board: &mut Array2D<Option<u32>>) -> bool {
    for x in 0..board.num_columns() {
        for y in 0..board.num_rows() {
            if board[(y, x)] == Some(number) {
                board[(y, x)] = None;
                return board.column_iter(x).all(|number| number.is_none())
                    || board.row_iter(y).all(|number| number.is_none())
            }
        }
    }
    return false
}

fn count_non_scratched(board: &Array2D<Option<u32>>) -> u32 {
    board.elements_column_major_iter().filter_map(|number| *number).sum()
}

#[cfg(test)]
mod tests {
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
