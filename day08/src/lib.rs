use std::collections::{HashMap, HashSet};

/// --- Day 8: Seven Segment Search ---
/// You barely reach the safety of the cave when the whale smashes into the cave mouth, collapsing it. Sensors indicate another exit to this cave at a much greater depth, so you have no choice but to press on.
///
/// As your submarine slowly makes its way through the cave system, you notice that the four-digit seven-segment displays in your submarine are malfunctioning; they must have been damaged during the escape. You'll be in a lot of trouble without them, so you'd better figure out what's wrong.
///
/// Each digit of a seven-segment display is rendered by turning on or off any of seven segments named a through g:
///
/// 0:      1:      2:      3:      4:
/// aaaa    ....    aaaa    aaaa    ....
/// b    c  .    c  .    c  .    c  b    c
/// b    c  .    c  .    c  .    c  b    c
/// ....    ....    dddd    dddd    dddd
/// e    f  .    f  e    .  .    f  .    f
/// e    f  .    f  e    .  .    f  .    f
/// gggg    ....    gggg    gggg    ....
///
/// 5:      6:      7:      8:      9:
/// aaaa    aaaa    aaaa    aaaa    aaaa
/// b    .  b    .  .    c  b    c  b    c
/// b    .  b    .  .    c  b    c  b    c
/// dddd    dddd    ....    dddd    dddd
/// .    f  e    f  .    f  e    f  .    f
/// .    f  e    f  .    f  e    f  .    f
/// gggg    gggg    ....    gggg    gggg
/// So, to render a 1, only segments c and f would be turned on; the rest would be off. To render a 7, only segments a, c, and f would be turned on.
///
/// The problem is that the signals which control the segments have been mixed up on each display. The submarine is still trying to display numbers by producing output on signal wires a through g, but those wires are connected to segments randomly. Worse, the wire/segment connections are mixed up separately for each four-digit display! (All of the digits within a display use the same connections, though.)
///
/// So, you might know that only signal wires b and g are turned on, but that doesn't mean segments b and g are turned on: the only digit that uses two segments is 1, so it must mean segments c and f are meant to be on. With just that information, you still can't tell which wire (b/g) goes to which segment (c/f). For that, you'll need to collect more information.
///
/// For each display, you watch the changing signals for a while, make a note of all ten unique signal patterns you see, and then write down a single four digit output value (your puzzle input). Using the signal patterns, you should be able to work out which pattern corresponds to which digit.
///
/// For example, here is what you might see in a single entry in your notes:
///
/// acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
/// cdfeb fcadb cdfeb cdbaf
/// (The entry is wrapped here to two lines so it fits; in your notes, it will all be on a single line.)
///
/// Each entry consists of ten unique signal patterns, a | delimiter, and finally the four digit output value. Within an entry, the same wire/segment connections are used (but you don't know what the connections actually are). The unique signal patterns correspond to the ten different ways the submarine tries to render a digit using the current wire/segment connections. Because 7 is the only digit that uses three segments, dab in the above example means that to render a 7, signal lines d, a, and b are on. Because 4 is the only digit that uses four segments, eafb means that to render a 4, signal lines e, a, f, and b are on.
///
/// Using this information, you should be able to work out which combination of signal wires corresponds to each of the ten digits. Then, you can decode the four digit output value. Unfortunately, in the above example, all of the digits in the output value (cdfeb fcadb cdfeb cdbaf) use five segments and are more difficult to deduce.
///
/// For now, focus on the easy digits. Consider this larger example:
///
/// be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |
/// fdgacbe cefdb cefbgd gcbe
/// edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |
/// fcgedb cgb dgebacf gc
/// fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |
/// cg cg fdcagb cbg
/// fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |
/// efabcd cedba gadfec cb
/// aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |
/// gecf egdcabf bgf bfgea
/// fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |
/// gebdcfa ecba ca fadegcb
/// dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |
/// cefg dcbef fcge gbcadfe
/// bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |
/// ed bcgafe cdgba cbgef
/// egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |
/// gbdfcae bgc cg cgb
/// gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |
/// fgae cfgab fg bagce
/// Because the digits 1, 4, 7, and 8 each use a unique number of segments, you should be able to tell which combinations of signals correspond to those digits. Counting only digits in the output values (the part after | on each line), in the above example, there are 26 instances of digits that use a unique number of segments (highlighted above).
///
/// In the output values, how many times do digits 1, 4, 7, or 8 appear?
pub fn part1(instructions: &[String]) -> usize {
    instructions
        .iter()
        .map(|instructions| {
            let parts = instructions.split(" | ").collect::<Vec<&str>>();

            // E.g.
            // bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |
            let mut inputs = parts[0]
                .split_whitespace()
                .map(|digit| digit.chars().collect::<HashSet<char>>())
                .collect::<Vec<HashSet<char>>>();

            // E.g. ed bcgafe cdgba cbgef
            let outputs = parts[1]
                .split_whitespace()
                .map(|digit| digit.chars().collect::<HashSet<char>>())
                .collect::<Vec<HashSet<char>>>();

            // 7-digit display segments
            //  aaaa
            // b    c
            // b    c
            //  dddd
            // e    f
            // e    f
            //  gggg

            //let mut segment_expected_to_actual_mapping: HashMap<char, char> = HashMap::new();

            let digit_1_pos = inputs.iter().position(|input| input.len() == 2).unwrap();
            let digit_1 = inputs.remove(digit_1_pos);

            let digit_4_pos = inputs.iter().position(|input| input.len() == 4).unwrap();
            let digit_4 = inputs.remove(digit_4_pos);

            let digit_7_pos = inputs.iter().position(|input| input.len() == 3).unwrap();
            let digit_7 = inputs.remove(digit_7_pos);

            let digit_8_pos = inputs.iter().position(|input| input.len() == 7).unwrap();
            let digit_8 = inputs.remove(digit_8_pos);

            return outputs
                .iter()
                .filter(|&output| *output == digit_1 || *output == digit_4 || *output == digit_7 || *output == digit_8)
                .count()
        })
        .sum()
}

pub fn part2(instructions: &[String]) -> usize {
    0
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        let sample_input: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let sample_output = 26;
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
