use itertools::Itertools;
use std::fmt;
use std::ops::{BitOr, Shl};

const INPUT: &str = include_str!("../input/day16.txt");

trait PeekableIterator: std::iter::Iterator {
    fn peek(&mut self) -> Option<&Self::Item>;
}

impl<I: std::iter::Iterator> PeekableIterator for std::iter::Peekable<I> {
    fn peek(&mut self) -> Option<&Self::Item> {
        std::iter::Peekable::peek(self)
    }
}

#[derive(Debug, Clone)]
struct Packet {
    version: u32,
    value: u64,
}

fn bytes_to_bits<T, B>(bytes: B) -> T
where
    T: num::Integer + num::NumCast + BitOr<T, Output = T> + Shl<T, Output = T> + fmt::Display,
    B: Iterator<Item = u8>,
{
    let mut bits = T::zero();
    for b in bytes {
        bits = (bits << T::one()) | T::from(b - b'0').unwrap();
    }
    bits
}

fn parse_operator_ops(
    iter: &mut impl PeekableIterator<Item = u8>,
    max_nb: Option<usize>,
    type_id: u8,
) -> Packet {
    let mut ops = match max_nb {
        Some(nb) => Vec::with_capacity(nb),
        None => Vec::new(),
    };
    while iter.peek().is_some() && (max_nb.is_none() || ops.len() < ops.capacity()) {
        ops.push(Packet::from_iterator(iter));
    }
    match type_id {
        0 => ops.into_iter().fold(Packet::default(), |acc, x| Packet {
            version: acc.version + x.version,
            value: acc.value + x.value,
        }),
        1 => ops.into_iter().fold(
            Packet {
                version: 0,
                value: 1,
            },
            |acc, x| {
                let version = acc.version + x.version;
                let value = acc.value * x.value;
                Packet { version, value }
            },
        ),
        2 => ops.into_iter().fold(
            Packet {
                version: 0,
                value: u64::MAX,
            },
            |acc, x| {
                let version = acc.version + x.version;
                let value = acc.value.min(x.value);
                Packet { version, value }
            },
        ),
        3 => ops.into_iter().fold(Packet::default(), |acc, x| {
            let version = acc.version + x.version;
            let value = acc.value.max(x.value);
            Packet { version, value }
        }),
        5 => {
            debug_assert_eq!(ops.len(), 2);
            Packet {
                version: ops[0].version + ops[1].version,
                value: (ops[0].value > ops[1].value) as u64,
            }
        }
        6 => {
            debug_assert_eq!(ops.len(), 2);
            Packet {
                version: ops[0].version + ops[1].version,
                value: (ops[0].value < ops[1].value) as u64,
            }
        }
        7 => {
            debug_assert_eq!(ops.len(), 2);
            Packet {
                version: ops[0].version + ops[1].version,
                value: (ops[0].value == ops[1].value) as u64,
            }
        }
        x => panic!("Unknown operation type id: {}", x),
    }
}

fn value_from_value_iter(iter: &mut impl PeekableIterator<Item = u8>) -> u64 {
    let mut val = 0;
    let mut done = false;
    while !done {
        done = iter
            .next()
            .expect("Unexpected end of stream while reading value group prefix bit")
            == b'0';
        val = (val << 4) | bytes_to_bits::<u64, _>(iter.by_ref().take(4));
    }
    val
}

fn packet_from_operator_iter(
    iter: &mut impl PeekableIterator<Item = u8>,
    type_id: u8,
    version: u32,
) -> Packet {
    let length_type_id = iter.next().expect("expected length type id");

    let packet = match length_type_id {
        b'1' => {
            let length = bytes_to_bits(iter.by_ref().take(11));
            parse_operator_ops(iter, Some(length), type_id)
        }
        b'0' => {
            let length: isize = bytes_to_bits(iter.by_ref().take(15));
            // error[E0275]: overflow evaluating the requirement `Peekable<std::str::Bytes>: Iterator`
            // let mut it = iter.take(length as usize).peekable();
            #[allow(clippy::needless_collect)]
            let col = iter.take(length as usize).collect::<Vec<_>>();
            let mut it = col.into_iter().peekable();
            parse_operator_ops(&mut it, None, type_id)
        }
        _ => unreachable!(),
    };
    Packet {
        value: packet.value,
        version: packet.version + version,
    }
}

impl Packet {
    fn default() -> Self {
        Self {
            value: 0,
            version: 0,
        }
    }

    fn from_iterator(iter: &mut impl PeekableIterator<Item = u8>) -> Self {
        let version = bytes_to_bits(iter.take(3));
        let packet_type = bytes_to_bits(iter.take(3));
        match packet_type {
            4 => Packet {
                version,
                value: value_from_value_iter(iter),
            },
            id => packet_from_operator_iter(iter, id, version),
        }
    }
}

fn parse(input: &'static str) -> Packet {
    let bin = input
        .trim()
        .chars()
        .map(|c| match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => panic!("invalid character"),
        })
        .join("");

    let mut iter = bin.bytes().peekable();
    Packet::from_iterator(&mut iter)
}

pub fn day16() -> (String, String) {
    let parsed = parse(INPUT);

    let part1 = parsed.version;
    let part2 = parsed.value;

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_value_packet_parsing() {
        let mut it = "110100101111111000101000".bytes().peekable();
        let packet = Packet::from_iterator(&mut it);
        assert_eq!(packet.version, 6);
        assert_eq!(packet.value, 2021);
    }

    #[test]
    fn test_part_1_test_input() {
        let parsed = parse("8A004A801A8002F478");
        assert_eq!(parsed.version, 16);
        let parsed = parse("620080001611562C8802118E34");
        assert_eq!(parsed.version, 12);
        let parsed = parse("A0016C880162017C3686B18A3D4780");
        assert_eq!(parsed.version, 31);
    }

    #[test]
    fn test_part_2_test_intput() {
        // sum
        let parsed = parse("C200B40A82");
        assert_eq!(parsed.value, 3);
        // product
        let parsed = parse("04005AC33890");
        assert_eq!(parsed.value, 54);
        // min
        let parsed = parse("880086C3E88112");
        assert_eq!(parsed.value, 7);
        // max
        let parsed = parse("CE00C43D881120");
        assert_eq!(parsed.value, 9);
        // less than
        let parsed = parse("D8005AC2A8F0");
        assert_eq!(parsed.value, 1);
        // greater than
        let parsed = parse("F600BC2D8F");
        assert_eq!(parsed.value, 0);
        // equal to
        let parsed = parse("9C005AC2F8F0");
        assert_eq!(parsed.value, 0);
        // sum, prod, equal
        let parsed = parse("9C0141080250320F1802104A08");
        assert_eq!(parsed.value, 1);
    }
}
