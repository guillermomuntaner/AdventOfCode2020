pub fn part1(input: &str) -> u32 {
    let mut iterator = input.chars().flat_map(to_binary);
    let (version_sum, _, _) = parse_packet(&mut iterator);
    version_sum
}

pub fn part2(input: &str) -> u64 {
    let mut iterator = input.chars().flat_map(to_binary);
    let (_, _, value) = parse_packet(&mut iterator);
    value
}

fn to_binary(hex: char) -> Vec<u32> {
    let mut bits: Vec<u32> = Vec::new();
    let mut decimal = hex.to_digit(16).unwrap();
    for _ in 0..4 {
        bits.insert(0,decimal % 2);
        decimal /= 2
    }
    bits
}

fn parse_packet(iterator: &mut impl Iterator<Item = u32>) -> (u32, u32, u64) {
    let mut version = 0;
    for _ in 0..3 {
        version = (version << 1) + iterator.next().unwrap()
    }

    let mut type_id = 0;
    for _ in 0..3 {
        type_id = (type_id << 1) + iterator.next().unwrap()
    }

    let mut version_sum = version;
    let mut body_bit_count = 0;

    let value = if type_id == 4 {
        // Literal value
        let mut literal_value = 0;
        loop {
            let group_prefix = iterator.next().unwrap();
            for _ in 0..4 {
                literal_value = (literal_value << 1) + iterator.next().unwrap()
            }
            body_bit_count += 5;
            if group_prefix == 0 {
                break;
            }
        }

        // Skip filling bits until multiple of 5
        let filling_zeros = body_bit_count % 5;
        for _ in 0..filling_zeros {
            iterator.next();
        }
        body_bit_count += filling_zeros;

        literal_value as u64
    } else {
        // Operator
        let lenght_type_id = iterator.next().unwrap();
        body_bit_count += 1;

        let values: Vec<u64> = match lenght_type_id {
            0 => {
                // If the length type ID is 0, then the next 15 bits are a number that represents
                // the total length in bits of the sub-packets contained by this packet.
                let mut length_in_bits = 0;
                for _ in 0..15 {
                    length_in_bits = (length_in_bits << 1) + iterator.next().unwrap()
                }
                body_bit_count += 15;

                let mut values: Vec<u64> = Vec::new();
                let mut sub_packages_bit_count = 0;
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
            1 => {
                // If the length type ID is 1, then the next 11 bits are a number that
                // represents the number of sub-packets immediately contained by this packet.
                let mut number_of_sub_packets = 0;
                for _ in 0..11 {
                    number_of_sub_packets = (number_of_sub_packets << 1) + iterator.next().unwrap()
                }
                body_bit_count += 11;

                let mut values = Vec::new();
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

        match type_id {
            0 => values.iter().sum(),
            1 => values.iter().product(),
            2 => *values.iter().min().unwrap(),
            3 => *values.iter().max().unwrap(),
            5 => (values[0] > values[1]) as u64,
            6 => (values[0] < values[1]) as u64,
            7 => (values[0] == values[1]) as u64,
            _ => panic!("Unexpected lenght_type_id={}", lenght_type_id),
        }
    };

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
