/// --- Day 3: Binary Diagnostic ---
/// The submarine has been making some odd creaking noises, so you ask it to produce a diagnostic report just in case.
///
/// The diagnostic report (your puzzle input) consists of a list of binary numbers which, when decoded properly, can tell you many useful things about the conditions of the submarine. The first parameter to check is the power consumption.
///
/// You need to use the binary numbers in the diagnostic report to generate two new binary numbers (called the gamma rate and the epsilon rate). The power consumption can then be found by multiplying the gamma rate by the epsilon rate.
///
/// Each bit in the gamma rate can be determined by finding the most common bit in the corresponding position of all numbers in the diagnostic report. For example, given the following diagnostic report:
///
/// 00100
/// 11110
/// 10110
/// 10111
/// 10101
/// 01111
/// 00111
/// 11100
/// 10000
/// 11001
/// 00010
/// 01010
/// Considering only the first bit of each number, there are five 0 bits and seven 1 bits. Since the most common bit is 1, the first bit of the gamma rate is 1.
///
/// The most common second bit of the numbers in the diagnostic report is 0, so the second bit of the gamma rate is 0.
///
/// The most common value of the third, fourth, and fifth bits are 1, 1, and 0, respectively, and so the final three bits of the gamma rate are 110.
///
/// So, the gamma rate is the binary number 10110, or 22 in decimal.
///
/// The epsilon rate is calculated in a similar way; rather than use the most common bit, the least common bit from each position is used. So, the epsilon rate is 01001, or 9 in decimal. Multiplying the gamma rate (22) by the epsilon rate (9) produces the power consumption, 198.
///
/// Use the binary numbers in your diagnostic report to calculate the gamma rate and epsilon rate, then multiply them together. What is the power consumption of the submarine? (Be sure to represent your answer in decimal, not binary.)
///
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
pub fn part1(input: &[String]) -> u32 {
    let bit_lenght = input.first().unwrap().len() as u32;
    let input = input
        .iter()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect::<Vec<u32>>();

    let mut gamma_rate: u32 = 0;
    let mut epsilon_rate: u32 = 0;

    for bit_pos in 0..bit_lenght {
        let mut ones_count = 0;
        for i in 0..input.len() {
            let bit = (input[i] >> bit_pos) & 1;
            ones_count += bit
        }
        if ones_count > (input.len() as u32) / 2 {
            gamma_rate += 1 << bit_pos
        } else {
            epsilon_rate += 1 << bit_pos
        }
    }

    gamma_rate * epsilon_rate
}

/// --- Part Two ---
/// Next, you should verify the life support rating, which can be determined by multiplying the oxygen generator rating by the CO2 scrubber rating.
///
/// Both the oxygen generator rating and the CO2 scrubber rating are values that can be found in your diagnostic report - finding them is the tricky part. Both values are located using a similar process that involves filtering out values until only one remains. Before searching for either rating value, start with the full list of binary numbers from your diagnostic report and consider just the first bit of those numbers. Then:
///
/// Keep only numbers selected by the bit criteria for the type of rating value for which you are searching. Discard numbers which do not match the bit criteria.
/// If you only have one number left, stop; this is the rating value for which you are searching.
/// Otherwise, repeat the process, considering the next bit to the right.
/// The bit criteria depends on which type of rating value you want to find:
///
/// To find oxygen generator rating, determine the most common value (0 or 1) in the current bit position, and keep only numbers with that bit in that position. If 0 and 1 are equally common, keep values with a 1 in the position being considered.
/// To find CO2 scrubber rating, determine the least common value (0 or 1) in the current bit position, and keep only numbers with that bit in that position. If 0 and 1 are equally common, keep values with a 0 in the position being considered.
/// For example, to determine the oxygen generator rating value using the same example diagnostic report from above:
///
/// Start with all 12 numbers and consider only the first bit of each number. There are more 1 bits (7) than 0 bits (5), so keep only the 7 numbers with a 1 in the first position: 11110, 10110, 10111, 10101, 11100, 10000, and 11001.
/// Then, consider the second bit of the 7 remaining numbers: there are more 0 bits (4) than 1 bits (3), so keep only the 4 numbers with a 0 in the second position: 10110, 10111, 10101, and 10000.
/// In the third position, three of the four numbers have a 1, so keep those three: 10110, 10111, and 10101.
/// In the fourth position, two of the three numbers have a 1, so keep those two: 10110 and 10111.
/// In the fifth position, there are an equal number of 0 bits and 1 bits (one each). So, to find the oxygen generator rating, keep the number with a 1 in that position: 10111.
/// As there is only one number left, stop; the oxygen generator rating is 10111, or 23 in decimal.
/// Then, to determine the CO2 scrubber rating value from the same example above:
///
/// Start again with all 12 numbers and consider only the first bit of each number. There are fewer 0 bits (5) than 1 bits (7), so keep only the 5 numbers with a 0 in the first position: 00100, 01111, 00111, 00010, and 01010.
/// Then, consider the second bit of the 5 remaining numbers: there are fewer 1 bits (2) than 0 bits (3), so keep only the 2 numbers with a 1 in the second position: 01111 and 01010.
/// In the third position, there are an equal number of 0 bits and 1 bits (one each). So, to find the CO2 scrubber rating, keep the number with a 0 in that position: 01010.
/// As there is only one number left, stop; the CO2 scrubber rating is 01010, or 10 in decimal.
/// Finally, to find the life support rating, multiply the oxygen generator rating (23) by the CO2 scrubber rating (10) to get 230.
///
/// Use the binary numbers in your diagnostic report to calculate the oxygen generator rating and CO2 scrubber rating, then multiply them together. What is the life support rating of the submarine? (Be sure to represent your answer in decimal, not binary.)
pub fn part2(input: &[String]) -> u32 {
    let bit_lenght = input.first().unwrap().len() as u32;

    let input = input
        .iter()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect::<Vec<u32>>();

    let mut bit_pos = bit_lenght;
    let mut input_filtered_for_oxygen = input.to_vec();
    while input_filtered_for_oxygen.len() != 1 {
        bit_pos -= 1;

        let ones_count = input_filtered_for_oxygen
            .iter()
            .map(|input| (*input >> bit_pos) & 1)
            .filter(|bit| *bit == 1)
            .count();

        let most_common_bit = if ones_count as f32 >= (input_filtered_for_oxygen.len() as f32) / 2.0 {
            1
        } else {
            0
        };

        input_filtered_for_oxygen.retain(|input| ((input >> bit_pos) & 1) == most_common_bit);
    }
    let oxygen_generator_rating = input_filtered_for_oxygen.first().unwrap();

    bit_pos = bit_lenght;
    let mut input_filtered_for_co2 = input.to_vec();
    while input_filtered_for_co2.len() != 1 {
        bit_pos -= 1;

        let ones_count = input_filtered_for_co2
            .iter()
            .map(|input| (*input >> bit_pos) & 1)
            .filter(|bit| *bit == 1)
            .count();

        let least_common_bit = if ones_count as f32 >= (input_filtered_for_co2.len() as f32) / 2.0 {
            0
        } else {
            1
        };

        input_filtered_for_co2.retain(|input| ((input >> bit_pos) & 1) == least_common_bit);
    }
    let co2_scrubber_rating = input_filtered_for_co2.first().unwrap();

    oxygen_generator_rating * co2_scrubber_rating
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let sample_input: Vec<String> = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001", "00010", "01010",
        ]
        .iter()
        .map(|line| line.to_string())
        .collect();
        let sample_output = 198;
        assert_eq!(crate::part1(&sample_input), sample_output);
    }

    #[test]
    fn test_part2() {
        let sample_input: Vec<String> = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001", "00010", "01010",
        ]
        .iter()
        .map(|line| line.to_string())
        .collect();
        let sample_output = 230;
        assert_eq!(crate::part2(&sample_input), sample_output);
    }
}
