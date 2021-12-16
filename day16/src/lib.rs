pub fn part1(input: &str) -> usize {
    let mut iterator = parse_input(input);
    let (version_sum, _, _) = parse_packet(&mut iterator);
    version_sum
}

pub fn part2(input: &str) -> usize {
    let mut iterator = parse_input(input);
    let (_, _, value) = parse_packet(&mut iterator);
    value
}

fn parse_input(input: &str) -> impl Iterator<Item = char> + '_ {
    input.chars().flat_map(|hex| match hex {
        '0' => vec!['0', '0', '0', '0'],
        '1' => vec!['0', '0', '0', '1'],
        '2' => vec!['0', '0', '1', '0'],
        '3' => vec!['0', '0', '1', '1'],
        '4' => vec!['0', '1', '0', '0'],
        '5' => vec!['0', '1', '0', '1'],
        '6' => vec!['0', '1', '1', '0'],
        '7' => vec!['0', '1', '1', '1'],
        '8' => vec!['1', '0', '0', '0'],
        '9' => vec!['1', '0', '0', '1'],
        'A' => vec!['1', '0', '1', '0'],
        'B' => vec!['1', '0', '1', '1'],
        'C' => vec!['1', '1', '0', '0'],
        'D' => vec!['1', '1', '0', '1'],
        'E' => vec!['1', '1', '1', '0'],
        'F' => vec!['1', '1', '1', '1'],
        _ => panic!("Unexpected char {}", hex),
    })
}

fn parse_packet(iterator: &mut impl Iterator<Item = char>) -> (usize, usize, usize) {
    let version = u8::from_str_radix(
        &vec![
            iterator.next().unwrap(),
            iterator.next().unwrap(),
            iterator.next().unwrap(),
        ]
        .iter()
        .collect::<String>(),
        2,
    )
    .unwrap();
    let type_id = u8::from_str_radix(
        &vec![
            iterator.next().unwrap(),
            iterator.next().unwrap(),
            iterator.next().unwrap(),
        ]
        .iter()
        .collect::<String>(),
        2,
    )
    .unwrap();

    let mut version_sum = version as usize;
    let mut body_bit_count = 0;
    let value;

    if type_id == 4 {
        // Literal value
        let mut literal_value_binary: Vec<char> = Vec::new();
        loop {
            let group_prefix = iterator.next().unwrap();
            literal_value_binary.push(iterator.next().unwrap());
            literal_value_binary.push(iterator.next().unwrap());
            literal_value_binary.push(iterator.next().unwrap());
            literal_value_binary.push(iterator.next().unwrap());
            body_bit_count += 5;
            if group_prefix == '0' {
                break;
            }
        }
        // Skip filling bits until multiple of 5
        for _ in 0..(body_bit_count % 5) {
            iterator.next();
            body_bit_count += 1;
        }

        value = usize::from_str_radix(&literal_value_binary.iter().collect::<String>(), 2).unwrap();
    } else {
        // Operator
        let lenght_type_id = iterator.next().unwrap();
        body_bit_count += 1;

        let values = match lenght_type_id {
            '0' => {
                // If the length type ID is 0, then the next 15 bits are a number that represents
                // the total length in bits of the sub-packets contained by this packet.
                let mut value: Vec<char> = Vec::new();
                for _ in 0..15 {
                    value.push(iterator.next().unwrap());
                    body_bit_count += 1;
                }
                let length_in_bits = usize::from_str_radix(&value.iter().collect::<String>(), 2).unwrap();

                let mut values: Vec<usize> = Vec::new();
                let mut sub_packages_bit_count = 0_usize;
                while sub_packages_bit_count < length_in_bits {
                    let (version, bit_count, value) = parse_packet(iterator);
                    sub_packages_bit_count += bit_count;
                    body_bit_count += bit_count;
                    version_sum += version;
                    values.push(value);
                }
                if sub_packages_bit_count != length_in_bits {
                    panic!(
                        "Lenght in bits don't match, expected {} but got {}",
                        length_in_bits, sub_packages_bit_count
                    );
                }
                values
            }
            '1' => {
                // If the length type ID is 1, then the next 11 bits are a number that
                // represents the number of sub-packets immediately contained by this packet.
                let mut value: Vec<char> = Vec::new();
                for _ in 0..11 {
                    value.push(iterator.next().unwrap());
                    body_bit_count += 1;
                }
                let number_of_sub_packets = u8::from_str_radix(&value.iter().collect::<String>(), 2).unwrap();

                let mut values: Vec<usize> = Vec::new();
                for _ in 0..number_of_sub_packets {
                    let (version, bit_count, value) = parse_packet(iterator);
                    body_bit_count += bit_count;
                    version_sum += version;
                    values.push(value);
                }
                values
            }
            _ => panic!("Unexpected lenght_type_id={}", lenght_type_id),
        };

        value = match type_id {
            // sum
            0 => values.iter().sum(),
            1 => values.iter().product(),
            2 => *values.iter().min().unwrap(),
            3 => *values.iter().max().unwrap(),
            5 => {
                if values[0] > values[1] {
                    1
                } else {
                    0
                }
            }
            6 => {
                if values[0] < values[1] {
                    1
                } else {
                    0
                }
            }
            7 => {
                if values[0] == values[1] {
                    1
                } else {
                    0
                }
            }
            _ => panic!("Unexpected lenght_type_id={}", lenght_type_id),
        }
    }

    (version_sum, 6 + body_bit_count, value)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_literal() {
        let input = "D2FE28";
        assert_eq!(crate::part1(input), 6);
    }

    #[test]
    fn test_part1_a() {
        let input = "8A004A801A8002F478";
        assert_eq!(crate::part1(input), 16);
    }

    #[test]
    fn test_part1_b() {
        let input = "620080001611562C8802118E34";
        assert_eq!(crate::part1(input), 12);
    }

    #[test]
    fn test_part1_c() {
        let input = "C0015000016115A2E0802F182340";
        assert_eq!(crate::part1(input), 23);
    }

    #[test]
    fn test_part1_v() {
        let input = "A0016C880162017C3686B18A3D4780";
        assert_eq!(crate::part1(input), 31);
    }

    //

    #[test]
    fn test_part2_a() {
        let input = "C200B40A82";
        assert_eq!(crate::part2(input), 3);
    }

    #[test]
    fn test_part2_b() {
        let input = "04005AC33890";
        assert_eq!(crate::part2(input), 54);
    }

    #[test]
    fn test_part2_c() {
        let input = "880086C3E88112";
        assert_eq!(crate::part2(input), 7);
    }

    #[test]
    fn test_part2_d() {
        let input = "CE00C43D881120";
        assert_eq!(crate::part2(input), 9);
    }

    #[test]
    fn test_part2_e() {
        let input = "D8005AC2A8F0";
        assert_eq!(crate::part2(input), 1);
    }

    #[test]
    fn test_part2_f() {
        let input = "F600BC2D8F";
        assert_eq!(crate::part2(input), 0);
    }

    #[test]
    fn test_part2_g() {
        let input = "9C005AC2F8F0";
        assert_eq!(crate::part2(input), 0);
    }

    #[test]
    fn test_part2_h() {
        let input = "9C0141080250320F1802104A08";
        assert_eq!(crate::part2(input), 1);
    }
}
