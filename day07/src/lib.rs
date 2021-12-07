/// --- Day 7: The Treachery of Whales ---
/// A giant whale has decided your submarine is its next meal, and it's much faster than you are. There's nowhere to run!
///
/// Suddenly, a swarm of crabs (each in its own tiny submarine - it's too deep for them otherwise) zooms in to rescue you! They seem to be preparing to blast a hole in the ocean floor; sensors indicate a massive underground cave system just beyond where they're aiming!
///
/// The crab submarines all need to be aligned before they'll have enough power to blast a large enough hole for your submarine to get through. However, it doesn't look like they'll be aligned before the whale catches you! Maybe you can help?
///
/// There's one major catch - crab submarines can only move horizontally.
///
/// You quickly make a list of the horizontal position of each crab (your puzzle input). Crab submarines have limited fuel, so you need to find a way to make all of their horizontal positions match while requiring them to spend as little fuel as possible.
///
/// For example, consider the following horizontal positions:
///
/// 16,1,2,0,4,2,7,1,2,14
/// This means there's a crab with horizontal position 16, a crab with horizontal position 1, and so on.
///
/// Each change of 1 step in horizontal position of a single crab costs 1 fuel. You could choose any horizontal position to align them all on, but the one that costs the least fuel is horizontal position 2:
///
/// Move from 16 to 2: 14 fuel
/// Move from 1 to 2: 1 fuel
/// Move from 2 to 2: 0 fuel
/// Move from 0 to 2: 2 fuel
/// Move from 4 to 2: 2 fuel
/// Move from 2 to 2: 0 fuel
/// Move from 7 to 2: 5 fuel
/// Move from 1 to 2: 1 fuel
/// Move from 2 to 2: 0 fuel
/// Move from 14 to 2: 12 fuel
/// This costs a total of 37 fuel. This is the cheapest possible outcome; more expensive outcomes include aligning at position 1 (41 fuel), position 3 (39 fuel), or position 10 (71 fuel).
///
/// Determine the horizontal position that the crabs can align to using the least fuel possible. How much fuel must they spend to align to that position?
/// --- Day 6: Lanternfish ---
/// The sea floor is getting steeper. Maybe the sleigh keys got carried this way?
///
/// A massive school of glowing lanternfish swims past. They must spawn quickly to reach such large numbers - maybe exponentially quickly? You should model their growth rate to be sure.
///
/// Although you know nothing about this specific species of lanternfish, you make some guesses about their attributes. Surely, each lanternfish creates a new lanternfish once every 7 days.
///
/// However, this process isn't necessarily synchronized between every lanternfish - one lanternfish might have 2 days left until it creates another lanternfish, while another might have 4. So, you can model each fish as a single number that represents the number of days until it creates a new lanternfish.
///
/// Furthermore, you reason, a new lanternfish would surely need slightly longer before it's capable of producing more lanternfish: two more days for its first cycle.
///
/// So, suppose you have a lanternfish with an internal timer value of 3:
///
/// After one day, its internal timer would become 2.
/// After another day, its internal timer would become 1.
/// After another day, its internal timer would become 0.
/// After another day, its internal timer would reset to 6, and it would create a new lanternfish with an internal timer of 8.
/// After another day, the first lanternfish would have an internal timer of 5, and the second lanternfish would have an internal timer of 7.
/// A lanternfish that creates a new fish resets its timer to 6, not 7 (because 0 is included as a valid timer value). The new lanternfish starts with an internal timer of 8 and does not start counting down until the next day.
///
/// Realizing what you're trying to do, the submarine automatically produces a list of the ages of several hundred nearby lanternfish (your puzzle input). For example, suppose you were given the following list:
///
/// 3,4,3,1,2
/// This list means that the first fish has an internal timer of 3, the second fish has an internal timer of 4, and so on until the fifth fish, which has an internal timer of 2. Simulating these fish over several days would proceed as follows:
///
/// Initial state: 3,4,3,1,2
/// After  1 day:  2,3,2,0,1
/// After  2 days: 1,2,1,6,0,8
/// After  3 days: 0,1,0,5,6,7,8
/// After  4 days: 6,0,6,4,5,6,7,8,8
/// After  5 days: 5,6,5,3,4,5,6,7,7,8
/// After  6 days: 4,5,4,2,3,4,5,6,6,7
/// After  7 days: 3,4,3,1,2,3,4,5,5,6
/// After  8 days: 2,3,2,0,1,2,3,4,4,5
/// After  9 days: 1,2,1,6,0,1,2,3,3,4,8
/// After 10 days: 0,1,0,5,6,0,1,2,2,3,7,8
/// After 11 days: 6,0,6,4,5,6,0,1,1,2,6,7,8,8,8
/// After 12 days: 5,6,5,3,4,5,6,0,0,1,5,6,7,7,7,8,8
/// After 13 days: 4,5,4,2,3,4,5,6,6,0,4,5,6,6,6,7,7,8,8
/// After 14 days: 3,4,3,1,2,3,4,5,5,6,3,4,5,5,5,6,6,7,7,8
/// After 15 days: 2,3,2,0,1,2,3,4,4,5,2,3,4,4,4,5,5,6,6,7
/// After 16 days: 1,2,1,6,0,1,2,3,3,4,1,2,3,3,3,4,4,5,5,6,8
/// After 17 days: 0,1,0,5,6,0,1,2,2,3,0,1,2,2,2,3,3,4,4,5,7,8
/// After 18 days: 6,0,6,4,5,6,0,1,1,2,6,0,1,1,1,2,2,3,3,4,6,7,8,8,8,8
/// Each day, a 0 becomes a 6 and adds a new 8 to the end of the list, while each other number decreases by 1 if it was present at the start of the day.
///
/// In this example, after 18 days, there are a total of 26 fish. After 80 days, there would be a total of 5934.
///
/// Find a way to simulate lanternfish. How many lanternfish would there be after 80 days?
pub fn part1(instructions: &[u64]) -> u64 {
    let min = instructions.iter().min().unwrap();
    let max = instructions.iter().max().unwrap();
    return (*min..=*max)
        .map(|position| {
            instructions
                .iter()
                .map(|val| abs_diff(position, *val))
                .sum::<u64>()
        })
        .min()
        .unwrap()
}

/// --- Part Two ---
/// The crabs don't seem interested in your proposed solution. Perhaps you misunderstand crab engineering?
///
/// As it turns out, crab submarine engines don't burn fuel at a constant rate. Instead, each change of 1 step in horizontal position costs 1 more unit of fuel than the last: the first step costs 1, the second step costs 2, the third step costs 3, and so on.
///
/// As each crab moves, moving further becomes more expensive. This changes the best horizontal position to align them all on; in the example above, this becomes 5:
///
/// Move from 16 to 5: 66 fuel
/// Move from 1 to 5: 10 fuel
/// Move from 2 to 5: 6 fuel
/// Move from 0 to 5: 15 fuel
/// Move from 4 to 5: 1 fuel
/// Move from 2 to 5: 6 fuel
/// Move from 7 to 5: 3 fuel
/// Move from 1 to 5: 10 fuel
/// Move from 2 to 5: 6 fuel
/// Move from 14 to 5: 45 fuel
/// This costs a total of 168 fuel. This is the new cheapest possible outcome; the old alignment position (2) now costs 206 fuel instead.
///
/// Determine the horizontal position that the crabs can align to using the least fuel possible so they can make you an escape route! How much fuel must they spend to align to that position?
pub fn part2(instructions: &[u64]) -> u64 {
    let min = instructions.iter().min().unwrap();
    let max = instructions.iter().max().unwrap();
    return (*min..=*max)
        .map(|position| {
            instructions
                .iter()
                .map(|val| fuel(abs_diff(position, *val)))
                .sum::<u64>()
        })
        .min()
        .unwrap()
}

fn abs_diff(slf: u64, other: u64)  -> u64 {
    if slf < other {
        other - slf
    } else {
        slf - other
    }
}

fn fuel(n: u64) -> u64 {
    if n < 2 {
        1
    } else {
        n + fuel(n - 1)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_part1() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let sample_input: Vec<u64> = input.split(',')
            .map(|number| number.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let sample_output = 37;
        assert_eq!(crate::part1(&sample_input), sample_output);
    }

    #[test]
    fn test_part2() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let sample_input: Vec<u64> = input.split(',')
            .map(|number| number.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let sample_output = 168;
        assert_eq!(crate::part2(&sample_input), sample_output);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(crate::fuel(1), 1);
        assert_eq!(crate::fuel(2), 2 + 1);
        assert_eq!(crate::fuel(3), 3 + 2 + 1);
        assert_eq!(crate::fuel(4), 4 + 3 + 2 + 1);
    }
}
