type SnailfishNumber = (SnailfishElement, SnailfishElement);

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum SnailfishElement {
    RegularNumber(u8),
    Pair(Box<SnailfishNumber>),
}

impl SnailfishElement {
    fn regular_number(self) -> u8 {
        if let SnailfishElement::RegularNumber(n) = self {
            n
        } else {
            panic!("Not a regular number")
        }
    }
}

pub fn part1(input: &str) -> usize {
    let sum = lines_sum(input);
    magnitude(sum)
}

pub fn part2(input: &str) -> usize {
    let lines = input.lines().map(|line| parse_line(line)).collect::<Vec<_>>();
    lines
        .clone()
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            lines
                .clone()
                .iter()
                .enumerate()
                .filter_map(|(j, b)| if j != i { Some(b) } else { None })
                .map(|b| sum(a.clone(), b.clone()))
                .map(magnitude)
                .collect::<Vec<_>>()
        })
        .max()
        .unwrap()
}

fn parse_line(input: &str) -> SnailfishNumber {
    let mut chars = input.chars();
    chars.next(); // Ignore opening bracket
    parse_number(&mut chars)
}

fn parse_number(iterator: &mut impl Iterator<Item = char>) -> SnailfishNumber {
    let left = parse_element(iterator);
    let comma = iterator.next().unwrap();
    assert_eq!(comma, ',');
    let right = parse_element(iterator);
    let closing = iterator.next().unwrap();
    assert_eq!(closing, ']');
    (left, right)
}

fn parse_element(iterator: &mut impl Iterator<Item = char>) -> SnailfishElement {
    let next = iterator.next().unwrap();
    if next == '[' {
        SnailfishElement::Pair(Box::new(parse_number(iterator)))
    } else {
        SnailfishElement::RegularNumber(next.to_digit(10).unwrap() as u8)
    }
}

#[derive(PartialEq, Debug)]
enum Result {
    Explode(u8, u8),
    Exploding(u8, u8, SnailfishNumber),
    ExplodeLeft(u8, SnailfishNumber),
    ExplodeRight(u8, SnailfishNumber),
    Modified(SnailfishNumber),
    None,
}

fn sum_to_leftmost(value: u8, number: SnailfishNumber) -> Option<SnailfishNumber> {
    let maybe_new_left_element = match number.0.clone() {
        SnailfishElement::RegularNumber(n) => Some(SnailfishElement::RegularNumber(n + value)),
        SnailfishElement::Pair(boxed_number) => {
            sum_to_leftmost(value, *boxed_number).map(|number| SnailfishElement::Pair(Box::new(number)))
        }
    };
    if let Some(new_left_element) = maybe_new_left_element {
        return Some((new_left_element, number.1));
    }

    let maybe_new_right_element = match number.1.clone() {
        SnailfishElement::RegularNumber(n) => Some(SnailfishElement::RegularNumber(n + value)),
        SnailfishElement::Pair(boxed_number) => {
            sum_to_rightmost(value, *boxed_number).map(|number| SnailfishElement::Pair(Box::new(number)))
        }
    };
    if let Some(new_right_element) = maybe_new_right_element {
        return Some((number.0, new_right_element));
    }
    None
}

fn sum_to_leftmost_element(element: &SnailfishElement, value: u8) -> Option<SnailfishElement> {
    match element {
        SnailfishElement::RegularNumber(number) => Some(SnailfishElement::RegularNumber(number + value)),
        SnailfishElement::Pair(boxed_number) => {
            sum_to_leftmost(value, *boxed_number.clone()).map(|new_number| SnailfishElement::Pair(Box::new(new_number)))
        }
    }
}

fn sum_to_rightmost(value: u8, number: SnailfishNumber) -> Option<SnailfishNumber> {
    let maybe_new_right_element = match number.1.clone() {
        SnailfishElement::RegularNumber(n) => Some(SnailfishElement::RegularNumber(n + value)),
        SnailfishElement::Pair(boxed_number) => {
            sum_to_rightmost(value, *boxed_number).map(|number| SnailfishElement::Pair(Box::new(number)))
        }
    };
    if let Some(new_right_element) = maybe_new_right_element {
        return Some((number.0, new_right_element));
    }

    let maybe_new_left_element = match number.0.clone() {
        SnailfishElement::RegularNumber(n) => Some(SnailfishElement::RegularNumber(n + value)),
        SnailfishElement::Pair(boxed_number) => {
            sum_to_rightmost(value, *boxed_number).map(|number| SnailfishElement::Pair(Box::new(number)))
        }
    };
    if let Some(new_left_element) = maybe_new_left_element {
        return Some((new_left_element, number.1));
    }
    None
}

fn sum_to_rightmost_element(element: &SnailfishElement, value: u8) -> Option<SnailfishElement> {
    match element {
        SnailfishElement::RegularNumber(number) => Some(SnailfishElement::RegularNumber(number + value)),
        SnailfishElement::Pair(boxed_number) => sum_to_rightmost(value, *boxed_number.clone())
            .map(|new_number| SnailfishElement::Pair(Box::new(new_number))),
    }
}

fn outer_reduce(number: &SnailfishNumber) -> Option<SnailfishNumber> {
    match reduce(number, Some(0)) {
        Result::Explode(_, _) => return Some(number.clone()),
        Result::Exploding(_, _, number) => return Some(number),
        Result::ExplodeLeft(_, number) => return Some(number),
        Result::ExplodeRight(_, number) => return Some(number),
        Result::Modified(number) => return Some(number),
        Result::None => {}
    }
    match reduce(number, None) {
        Result::Explode(_, _) => Some(number.clone()),
        Result::Exploding(_, _, number) => Some(number),
        Result::ExplodeLeft(_, number) => Some(number),
        Result::ExplodeRight(_, number) => Some(number),
        Result::Modified(number) => Some(number),
        Result::None => None,
    }
}

fn reduce(number: &SnailfishNumber, nested_level: Option<u8>) -> Result {
    let (left_element, right_element) = number;

    if let Some(nested_level) = nested_level {
        if nested_level == 4 {
            return Result::Explode(number.0.clone().regular_number(), number.1.clone().regular_number());
        }
    }

    match left_element {
        SnailfishElement::RegularNumber(n) => {
            if None == nested_level && n >= &10 {
                let split_left = n / 2;
                let split_right = n / 2 + (n % 2 != 0) as u8;
                let new_left_element = SnailfishElement::Pair(Box::new((
                    SnailfishElement::RegularNumber(split_left),
                    SnailfishElement::RegularNumber(split_right),
                )));
                return Result::Modified((new_left_element, right_element.clone()));
            }
        }
        SnailfishElement::Pair(boxed_number) => {
            match reduce(&**boxed_number, nested_level.map(|l| l + 1)) {
                Result::Explode(left_value, right_value) => {
                    let new_left_element = SnailfishElement::RegularNumber(0);
                    // Try to add it on the right branch
                    return match sum_to_leftmost_element(right_element, right_value) {
                        Some(new_right_element) => {
                            Result::ExplodeLeft(left_value, (new_left_element, new_right_element))
                        }
                        None => Result::Exploding(left_value, right_value, (new_left_element, right_element.clone())),
                    };
                }
                Result::Exploding(left_value, right_value, new_left_number) => {
                    let new_left_element = SnailfishElement::Pair(Box::new(new_left_number));
                    // Try to add it on the right branch
                    return match sum_to_leftmost_element(right_element, right_value) {
                        Some(new_right_element) => {
                            Result::ExplodeLeft(left_value, (new_left_element, new_right_element))
                        }
                        None => Result::Exploding(left_value, right_value, (new_left_element, right_element.clone())),
                    };
                }
                Result::ExplodeLeft(left_value, new_left_number) => {
                    // Cannot fix it, just forward up in the hierarchy
                    let new_left_element = SnailfishElement::Pair(Box::new(new_left_number));
                    return Result::ExplodeLeft(left_value, (new_left_element, right_element.clone()));
                }
                Result::ExplodeRight(right_value, new_left_number) => {
                    let new_left_element = SnailfishElement::Pair(Box::new(new_left_number));
                    return match sum_to_leftmost_element(right_element, right_value) {
                        Some(new_right_element) => Result::Modified((new_left_element, new_right_element)),
                        None => Result::ExplodeRight(right_value, (new_left_element, right_element.clone())),
                    };
                }
                Result::Modified(new_left_number) => {
                    let new_left_element = SnailfishElement::Pair(Box::new(new_left_number));
                    return Result::Modified((new_left_element, right_element.clone()));
                }
                Result::None => {}
            }
        }
    }

    match right_element {
        SnailfishElement::RegularNumber(n) => {
            if None == nested_level && n >= &10 {
                let split_left = n / 2;
                let split_right = n / 2 + (n % 2 != 0) as u8;
                let new_right_element = SnailfishElement::Pair(Box::new((
                    SnailfishElement::RegularNumber(split_left),
                    SnailfishElement::RegularNumber(split_right),
                )));
                return Result::Modified((left_element.clone(), new_right_element));
            }
        }
        SnailfishElement::Pair(boxed_number) => {
            match reduce(&**boxed_number, nested_level.map(|l| l + 1)) {
                Result::Explode(left_value, right_value) => {
                    let new_right_element = SnailfishElement::RegularNumber(0);
                    // Try to add it on the right branch
                    return match sum_to_rightmost_element(left_element, left_value) {
                        Some(new_left_element) => {
                            Result::ExplodeRight(right_value, (new_left_element, new_right_element))
                        }
                        None => Result::Exploding(left_value, right_value, (left_element.clone(), new_right_element)),
                    };
                }
                Result::Exploding(left_value, right_value, new_right_number) => {
                    let new_right_element = SnailfishElement::Pair(Box::new(new_right_number));
                    // Try to add it on the right branch
                    return match sum_to_rightmost_element(left_element, left_value) {
                        Some(new_left_element) => {
                            Result::ExplodeRight(right_value, (new_left_element, new_right_element))
                        }
                        None => Result::Exploding(left_value, right_value, (left_element.clone(), new_right_element)),
                    };
                }
                Result::ExplodeLeft(left_value, new_right_number) => {
                    let new_right_element = SnailfishElement::Pair(Box::new(new_right_number));
                    return match sum_to_rightmost_element(left_element, left_value) {
                        Some(new_left_element) => Result::Modified((new_left_element, new_right_element)),
                        None => Result::ExplodeLeft(left_value, (left_element.clone(), new_right_element)),
                    };
                }
                Result::ExplodeRight(right_value, new_right_number) => {
                    // Cannot fix it, just forward up in the hierarchy
                    let new_right_element = SnailfishElement::Pair(Box::new(new_right_number));
                    return Result::ExplodeRight(right_value, (left_element.clone(), new_right_element));
                }
                Result::Modified(new_right_number) => {
                    let new_right_element = SnailfishElement::Pair(Box::new(new_right_number));
                    return Result::Modified((left_element.clone(), new_right_element));
                }
                Result::None => {}
            }
        }
    }

    Result::None
}

fn sum(a: SnailfishNumber, b: SnailfishNumber) -> SnailfishNumber {
    let mut snailfish_number = (SnailfishElement::Pair(Box::new(a)), SnailfishElement::Pair(Box::new(b)));
    while let Some(reduced_snailfish_number) = outer_reduce(&snailfish_number) {
        snailfish_number = reduced_snailfish_number
    }
    snailfish_number
}

fn magnitude(snailfish_number: SnailfishNumber) -> usize {
    let (left, right) = snailfish_number;
    let left_value = match left {
        SnailfishElement::RegularNumber(value) => value as usize,
        SnailfishElement::Pair(boxed_value) => magnitude(*boxed_value),
    };
    let right_value = match right {
        SnailfishElement::RegularNumber(value) => value as usize,
        SnailfishElement::Pair(boxed_value) => magnitude(*boxed_value),
    };
    3 * left_value + 2 * right_value
}

fn lines_sum(input: &str) -> SnailfishNumber {
    input.lines().map(|line| parse_line(line)).reduce(sum).unwrap()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_parse() {
        crate::parse_line("[1,2]");
        crate::parse_line("[[1,2],3]");
        crate::parse_line("[9,[8,7]]");
        crate::parse_line("[[1,9],[8,5]]");
        crate::parse_line("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]");
        crate::parse_line("[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]");
        crate::parse_line("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]");
    }

    #[test]
    fn test_explosion_a() {
        let number = crate::parse_line("[[[[[9,8],1],2],3],4]");
        let reduced = crate::outer_reduce(&number);
        let expected = Some(crate::parse_line("[[[[0,9],2],3],4]"));
        assert_eq!(reduced, expected);
    }

    #[test]
    fn test_explosion_b() {
        let number = crate::parse_line("[7,[6,[5,[4,[3,2]]]]]");
        let reduced = crate::outer_reduce(&number);
        let expected = Some(crate::parse_line("[7,[6,[5,[7,0]]]]"));
        assert_eq!(reduced, expected);
    }

    #[test]
    fn test_explosion_c() {
        let number = crate::parse_line("[[6,[5,[4,[3,2]]]],1]");
        let reduced = crate::outer_reduce(&number);
        let expected = Some(crate::parse_line("[[6,[5,[7,0]]],3]"));
        assert_eq!(reduced, expected);
    }

    #[test]
    fn test_explosion_d() {
        let number = crate::parse_line("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        let reduced = crate::outer_reduce(&number);
        let expected = Some(crate::parse_line("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"));
        assert_eq!(reduced, expected);
    }

    #[test]
    fn test_explosion_e() {
        let number = crate::parse_line("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        let reduced = crate::outer_reduce(&number);
        let expected = Some(crate::parse_line("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"));
        assert_eq!(reduced, expected);
    }

    #[test]
    fn test_split() {
        let a = crate::parse_line("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let b = crate::parse_line("[1,1]");
        let reduced = crate::sum(a, b);
        let expected = crate::parse_line("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        assert_eq!(reduced, expected);
    }

    #[test]
    fn test_magnitudes() {
        assert_eq!(crate::magnitude(crate::parse_line("[9,1]")), 29);
        assert_eq!(crate::magnitude(crate::parse_line("[[1,2],[[3,4],5]]")), 143);
        assert_eq!(
            crate::magnitude(crate::parse_line("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")),
            1384
        );
        assert_eq!(
            crate::magnitude(crate::parse_line("[[[[1,1],[2,2]],[3,3]],[4,4]]")),
            445
        );
        assert_eq!(
            crate::magnitude(crate::parse_line("[[[[3,0],[5,3]],[4,4]],[5,5]]")),
            791
        );
        assert_eq!(
            crate::magnitude(crate::parse_line("[[[[5,0],[7,4]],[5,5]],[6,6]]")),
            1137
        );
        assert_eq!(
            crate::magnitude(crate::parse_line(
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
            )),
            3488
        );
        assert_eq!(
            crate::magnitude(crate::parse_line(
                "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"
            )),
            4140
        );
    }

    #[test]
    fn test_sample_list_sum_1() {
        let input = crate::lines_sum(
            "[1,1]
[2,2]
[3,3]
[4,4]",
        );
        assert_eq!(input, crate::parse_line("[[[[1,1],[2,2]],[3,3]],[4,4]]"));
    }

    #[test]
    fn test_sample_list_sum_2() {
        let input = crate::lines_sum(
            "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]",
        );
        assert_eq!(input, crate::parse_line("[[[[3,0],[5,3]],[4,4]],[5,5]]"));
    }

    #[test]
    fn test_sample_list_sum_3() {
        let input = crate::lines_sum(
            "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]",
        );
        assert_eq!(input, crate::parse_line("[[[[5,0],[7,4]],[5,5]],[6,6]]"));
    }

    #[test]
    fn test_sample_list_sum_4() {
        let input = crate::lines_sum(
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]",
        );
        assert_eq!(
            input,
            crate::parse_line("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
        );
    }

    #[test]
    fn test_part1() {
        let input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        assert_eq!(crate::part1(input), 4140);
    }

    #[test]
    fn test_part2() {
        let input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        assert_eq!(crate::part2(input), 3993);
    }
}
