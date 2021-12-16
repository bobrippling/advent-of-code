fn main() -> Result<(), Box<dyn std::error::Error>> {
    let packet = std::fs::read_to_string("input.txt")?.trim().parse()?;

    println!("Part 1: {}", part1(&packet));
    println!("Part 2: {}", part2(&packet));

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
struct Header {
    version: u8, // 3 bits
    type_id: u8, // 3 bits
}

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Literal(u8, u64), // version, literal num
    Operator(Header, Vec<Packet>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Bits {
    bits: Vec<bool>,
}

enum Types {
    Literal = 4,
}

fn part1(packet: &Packet) -> usize {
    packet.version_sum()
}

fn part2(packet: &Packet) -> usize {
    packet.value()
}

impl Packet {
    fn version_sum(&self) -> usize {
        use Packet::*;

        match *self {
            Literal(version, _) => version as _,
            Operator(Header { version, .. }, ref packets) => {
                version as usize + packets.iter().map(Packet::version_sum).sum::<usize>()
            }
        }
    }

    fn value(&self) -> usize {
        use Packet::*;

        match *self {
            Literal(_, n) => n as _,
            Operator(Header { type_id, .. }, ref packets) => {
                let values = packets.iter().map(Packet::value);

                let expect_two = || {
                    if let [a, b] = values.clone().collect::<Vec<_>>()[..] {
                        (a, b)
                    } else {
                        panic!("expected exactly two sub-packets");
                    }
                };

                match type_id {
                    0 => values.sum(),
                    1 => values.product(),
                    2 => values.min().unwrap(),
                    3 => values.max().unwrap(),
                    5 => {
                        let (a, b) = expect_two();
                        (a > b) as _
                    }
                    6 => {
                        let (a, b) = expect_two();
                        (a < b) as _
                    }
                    7 => {
                        let (a, b) = expect_two();
                        (a == b) as _
                    }
                    _ => panic!("unknown type_id {}", type_id),
                }
            }
        }
    }
}

impl std::str::FromStr for Packet {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bits = s.parse()?;

        let (packet, _) = parse_packet(bits).map_err(|_| "couldn't parse packet")?;
        Ok(packet)
    }
}

fn parse_packet(bits: Bits) -> Result<(Packet, Bits), Bits> {
    let iter = bits.iter();
    let version = iter.take(3).collect_u64();
    let type_id = iter.skip(3).take(3).collect_u64();

    let header = Header {
        version: version as _,
        type_id: type_id as _,
    };

    let rest = iter.skip(6);

    if type_id == Types::Literal as _ {
        let mut literal_bits = 0;
        let mut literal = 0;

        for chunk in rest.clone().collect::<Bits>().chunks(5) {
            let n = chunk[1..].iter().copied().collect_u64();

            literal = (literal << 4) | n;
            literal_bits += 5;

            if chunk[0] == false {
                break;
            }
        }

        let remaining = rest.clone().skip(literal_bits).collect();

        Ok((Packet::Literal(version as _, literal), remaining))
    } else {
        let length_type_id = rest
            .clone()
            .take(1)
            .next()
            .ok_or_else(|| rest.clone().collect::<Bits>())?;

        if length_type_id == false {
            // 15 bit length of sub packets
            let len = rest.clone().skip(1).take(15).collect_u64() as usize;

            let mut sub: Bits = rest.clone().skip(1 + 15).take(len as usize).collect();
            let mut packets = vec![];

            while sub.len() > 0 {
                if let Ok((packet, remaining)) = parse_packet(sub) {
                    packets.push(packet);
                    sub = remaining;
                } else {
                    panic!("wrong bit length")
                }
            }

            let remaining = rest.clone().skip(1 + 15 + len).collect();
            Ok((Packet::Operator(header, packets), remaining))
        } else {
            // 11 bit sub-packet count
            let count = rest.clone().skip(1).take(11).collect_u64() as usize;
            let mut sub: Bits = rest.clone().skip(1 + 11).collect();
            let mut packets = vec![];

            while packets.len() != count {
                if let Ok((packet, remaining)) = parse_packet(sub.clone()) {
                    packets.push(packet);
                    sub = remaining;
                } else {
                    break;
                }
            }

            Ok((Packet::Operator(header, packets), sub))
        }
    }
}

impl std::str::FromStr for Bits {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v = vec![];

        for ch in s.chars() {
            let hex = match ch {
                '0'..='9' => ch as u8 - b'0',
                'A'..='F' => ch as u8 - b'A' + 10,
                _ => return Err("unrecognised char"),
            };

            for i in (0..4).rev() {
                v.push((hex & (1 << i)) != 0);
            }
        }

        Ok(Self { bits: v })
    }
}

impl Bits {
    fn iter(&self) -> BitsIter {
        BitsIter {
            bit: 0, // MSB first
            bits: self,
        }
    }

    #[cfg(test)]
    fn empty_with(n: usize) -> Self {
        Self {
            bits: vec![false; n],
        }
    }

    fn chunks(&self, n: usize) -> impl Iterator<Item = &[bool]> {
        self.bits.chunks(n)
    }

    fn len(&self) -> usize {
        self.bits.len()
    }
}

#[derive(Clone, Copy)]
struct BitsIter<'b> {
    bit: usize,
    bits: &'b Bits,
}

impl Iterator for BitsIter<'_> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.bits.bits.get(self.bit).copied();
        self.bit += 1;
        out
    }
}

trait Collect {
    fn collect_u64(self) -> u64;
}

impl<I> Collect for I
where
    I: Iterator<Item = bool>,
{
    fn collect_u64(self) -> u64 {
        self.map(|b| b as u64).fold(0, |acc, n| (acc << 1) | n)
    }
}

impl From<(u64, usize)> for Bits {
    fn from((n, count): (u64, usize)) -> Self {
        let mut bits = vec![];

        for i in (0..count).rev() {
            bits.push((n & (1 << i)) != 0);
        }

        Bits { bits }
    }
}

impl FromIterator<bool> for Bits {
    fn from_iter<T: IntoIterator<Item = bool>>(iter: T) -> Self {
        Self {
            bits: iter.into_iter().collect(),
        }
    }
}

impl std::fmt::Binary for Bits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        for bit in self.iter() {
            write!(f, "{}", if bit { '1' } else { '0' })?;
        }

        write!(f, "]")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_collect_u64() {
        let a = [true, false, true];

        assert_eq!(a.iter().copied().collect_u64(), 0b101);
    }

    #[test]
    fn test_collect_bits_small() {
        let ents = [true, false, true];
        let bits: Bits = ents.into_iter().collect();
        assert_eq!(
            bits,
            Bits {
                bits: vec![true, false, true],
            }
        );
    }

    #[test]
    fn test_from_u64() {
        let bits = Bits::from((0b10110, 5));

        assert_eq!(
            bits,
            Bits {
                bits: vec![true, false, true, true, false],
            }
        );
    }

    #[test]
    fn test_bits() {
        let bits: Bits = "D2FE28".parse().unwrap();

        assert_eq!(
            bits,
            Bits {
                bits: [1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0]
                    .into_iter()
                    .map(|i| i != 0)
                    .collect()
            }
        )
    }

    #[test]
    fn test_header() {
        let bits = "D2FE28".parse().unwrap();
        let (packet, left) = parse_packet(bits).unwrap();

        match packet {
            Packet::Literal(version, literal) => {
                assert_eq!(version, 0b110);
                assert_eq!(literal, 0b_0111_1110_0101);

                assert_eq!(left, Bits::empty_with(3));
            }
            _ => panic!("expected literal"),
        }
    }

    #[test]
    fn test_operator_packet() {
        fn packet_test(s: &str, expected_packet: Packet, expected_left: Bits) {
            let bits = s.parse().unwrap();
            let (packet, left) = parse_packet(bits).unwrap();

            assert_eq!(packet, expected_packet);
            assert_eq!(left, expected_left);
        }

        packet_test(
            "38006F45291200",
            Packet::Operator(
                Header {
                    version: 1,
                    type_id: 6,
                },
                vec![Packet::Literal(6, 10), Packet::Literal(2, 20)],
            ),
            Bits::empty_with(7),
        );

        packet_test(
            "EE00D40C823060",
            Packet::Operator(
                Header {
                    version: 7,
                    type_id: 3,
                },
                vec![
                    Packet::Literal(2, 1),
                    Packet::Literal(4, 2),
                    Packet::Literal(1, 3),
                ],
            ),
            Bits::empty_with(5),
        );

        packet_test(
            "9C0141080250320F1802104A08",
            Packet::Operator(
                Header {
                    version: 4,
                    type_id: 7,
                },
                vec![
                    Packet::Operator(
                        Header {
                            version: 2,
                            type_id: 0,
                        },
                        vec![Packet::Literal(2, 1), Packet::Literal(4, 3)],
                    ),
                    Packet::Operator(
                        Header {
                            version: 6,
                            type_id: 1,
                        },
                        vec![Packet::Literal(0, 2), Packet::Literal(2, 2)],
                    ),
                ],
            ),
            Bits::empty_with(2),
        );
    }

    #[test]
    fn test_part1() {
        let examples = [
            ("8A004A801A8002F478", 16),
            ("620080001611562C8802118E34", 12),
            ("C0015000016115A2E0802F182340", 23),
            ("A0016C880162017C3686B18A3D4780", 31),
        ];

        for (s, n) in examples {
            let packets = s.parse().unwrap();
            assert_eq!(part1(&packets), n);
        }
    }

    #[test]
    fn test_part2() {
        let examples = [
            ("C200B40A82", 3),
            ("04005AC33890", 54),
            ("880086C3E88112", 7),
            ("CE00C43D881120", 9),
            ("D8005AC2A8F0", 1),
            ("F600BC2D8F", 0),
            ("9C005AC2F8F0", 0),
            ("9C0141080250320F1802104A08", 1),
        ];

        for (s, n) in examples {
            let packets = s.parse().unwrap();
            assert_eq!(part2(&packets), n);
        }
    }
}
