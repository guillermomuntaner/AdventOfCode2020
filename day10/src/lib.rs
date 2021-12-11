pub fn part1(str: &str) -> usize {
    let mut score = 0;

    fn seek<I>(close_char: Option<char>, chars: &mut I) -> Option<usize>
    where
        I: Iterator<Item = char>,
    {
        loop {
            let c = chars.next();
            match c {
                None => return None,
                Some('(') => match seek(Some(')'), chars) {
                    Some(score) => return Some(score),
                    _ => {}
                },
                Some('[') => match seek(Some(']'), chars) {
                    Some(score) => return Some(score),
                    _ => {}
                },
                Some('{') => match seek(Some('}'), chars) {
                    Some(score) => return Some(score),
                    _ => {}
                },
                Some('<') => match seek(Some('>'), chars) {
                    Some(score) => return Some(score),
                    _ => {}
                },
                Some(')') => {
                    if Some(')') != close_char {
                        return Some(3);
                    } else {
                        return None;
                    }
                }
                Some(']') => {
                    if Some(']') != close_char {
                        return Some(57);
                    } else {
                        return None;
                    }
                }
                Some('}') => {
                    if Some('}') != close_char {
                        return Some(1197);
                    } else {
                        return None;
                    }
                }
                Some('>') => {
                    if Some('>') != close_char {
                        return Some(25137);
                    } else {
                        return None;
                    }
                }
                _ => panic!("Unexpected character"),
            }
        }
    }

    for line in str.lines() {
        let mut iter = line.chars();
        score += seek(None, &mut iter).unwrap_or(0);
    }

    score
}

pub fn part2(str: &str) -> usize {
    fn seek<I>(close_char: Option<char>, chars: &mut I) -> Option<usize>
    where
        I: Iterator<Item = char>,
    {
        let mut acc_score = 0;
        loop {
            let c = chars.next();
            match c {
                None => match close_char {
                    None => return Some(acc_score),
                    Some(')') => return Some(5 * acc_score + 1),
                    Some(']') => return Some(5 * acc_score + 2),
                    Some('}') => return Some(5 * acc_score + 3),
                    Some('>') => return Some(5 * acc_score + 4),
                    Some(_) => panic!("Unexpected character"),
                },
                Some('(') => match seek(Some(')'), chars) {
                    Some(score) => acc_score += score,
                    None => return None,
                },
                Some('[') => match seek(Some(']'), chars) {
                    Some(score) => acc_score += score,
                    None => return None,
                },
                Some('{') => match seek(Some('}'), chars) {
                    Some(score) => acc_score += score,
                    None => return None,
                },
                Some('<') => match seek(Some('>'), chars) {
                    Some(score) => acc_score += score,
                    None => return None,
                },
                Some(')') => {
                    if Some(')') != close_char {
                        return None;
                    } else {
                        return Some(0);
                    }
                }
                Some(']') => {
                    if Some(']') != close_char {
                        return None;
                    } else {
                        return Some(0);
                    }
                }
                Some('}') => {
                    if Some('}') != close_char {
                        return None;
                    } else {
                        return Some(0);
                    }
                }
                Some('>') => {
                    if Some('>') != close_char {
                        return None;
                    } else {
                        return Some(0);
                    }
                }
                _ => panic!("Unexpected character"),
            }
        }
    }

    let mut scores: Vec<usize> = Vec::new();
    for instruction in str.lines() {
        let mut iter = instruction.chars();
        match seek(None, &mut iter) {
            Some(score) => scores.push(score),
            None => {}
        }
    }

    scores.sort();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        assert_eq!(crate::part1(input), 26397);
    }

    #[test]
    fn test_part2() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        assert_eq!(crate::part2(input), 288957);
    }
}
