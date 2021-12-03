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

    let input_length = input.len() as u32;

    let mut gamma_rate: u32 = 0;
    let mut epsilon_rate: u32 = 0;

    for bit_pos in 0..bit_lenght {
        let mut ones_count = 0;
        for i in 0..input.len() {
            let bit = (input[i] >> bit_pos) & 1;
            println!("There is a {:b} at bit pos {} in the input #{}", bit, bit_pos, i);
            ones_count += bit
        }
        println!("Counted {} 1s in {} inputs", ones_count, input.len());
        if ones_count > (input.len() as u32)/2 {
            gamma_rate += (1 << bit_pos)
        } else {
            epsilon_rate += (1 << bit_pos)
        }
    }

    println!("Gamma rate= {} & epsilon rate= {}", gamma_rate, epsilon_rate);

    gamma_rate * epsilon_rate
}


#[cfg(test)]
mod tests_part1 {
    #[test]
    fn test_part1() {
        let sample_input: Vec<String> = vec!["00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001", "00010", "01010"]
            .iter().map(|line| line.to_string()).collect();
        let sample_output = 198;
        assert_eq!(crate::part1(&sample_input), sample_output);
    }
}
