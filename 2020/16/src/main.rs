use std::fs;
use std::ops;
use std::collections::HashMap;

use regex::Regex;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let info = parse(&input);
    println!("Part 1: {}", part1(&info));
    println!("Part 2: {}", part2(&info));
}

fn part2(info: &Info) -> u64 {
    struct Possibility(u32);
    impl Possibility {
        fn zero_bit(&mut self, bit: usize) {
            self.0 &= !(1 << bit);
        }
    }

    let nvals = info.my_ticket.values.len();
    let mut field_possibilities = HashMap::new();
    for (i, _field) in info.field_rules.iter().enumerate() {
        field_possibilities.insert(
            i,
            Possibility(
                !(!0 << nvals)
            )
        );
    }

    let valid_tickets = info.nearby_tickets
        .iter()
        .filter(|ticket| ticket.is_valid(&info.field_rules))
        .collect::<Vec<_>>();

    for (rule_i, FieldRule { rule, .. }) in info.field_rules.iter().enumerate() {
        let possibility = field_possibilities.get_mut(&rule_i).unwrap();

        for ticket in &valid_tickets {
            for (val_index, &val) in ticket.values.iter().enumerate() {
                if !rule.matches(val) {
                    possibility.zero_bit(val_index);
                }
            }
        }
    }

    #[cfg(feature = "show-working")]
    {
        println!("pre masking:");
        for (&i, &Possibility(mask)) in field_possibilities.iter() {
            println!(
                "  mask[{:02}]: {:020b} // possibilities for \"{}\"",
                i,
                mask,
                info.field_rules[i as usize].field,
            );
        }
    }

    loop {
        let mut did_mask = false;
        for i in 0..info.field_rules.len() {
            let &Possibility(mask) = field_possibilities.get(&i).unwrap();

            if mask.has_single_bit() {
                // single choice, remove from all the others
                field_possibilities
                    .iter_mut()
                    .filter(|(&j, _)| j != i)
                    .for_each(|(_, possibility)| {
                        let old = possibility.0;
                        possibility.0 &= !mask;

                        did_mask |= old != possibility.0;
                    })
            }
        }

        if !did_mask {
            break;
        }
    }

    #[cfg(feature = "show-working")]
    {
        println!("post masking:");
        for (&i, &Possibility(mask)) in field_possibilities.iter() {
            println!(
                "  mask[{:02}]: {:020b} // possibilities for \"{}\"",
                i,
                mask,
                info.field_rules[i as usize].field,
            );
        }
    }

    let mut field_corrections = HashMap::<u32, u32>::new();
    for (&i, &Possibility(mask)) in field_possibilities.iter() {
        if !mask.has_single_bit() {
            panic!("couldn't narrow down mask[{}] enough: {:b}", i, mask);
        }

        let index = mask.first_bit_set().unwrap() as u32;

        field_corrections.insert(index, i as u32);
    }

    #[cfg(feature = "show-working")]
    {
        println!("field_corrections:");
        for (&from, &to) in field_corrections.iter() {
            println!(
                "  field {} (\"{}\") is \"{}\"",
                from,
                info.field_rules[from as usize].field,
                info.field_rules[to as usize].field,
            );
        }
    }

    let mut mul = 1;
    for (&orig_i, &field_i) in field_corrections.iter() {
        let field_rule = &info.field_rules[field_i as usize];

        if field_rule.field.starts_with("departure") {
            let my_ticket_val = info.my_ticket.values[orig_i as usize];

            #[cfg(feature = "show-working")]
            println!(
                "field \"{}\" matches, map index {} -> {} giving value {}",
                field_rule.field,
                field_i,
                orig_i,
                my_ticket_val
            );

            mul *= my_ticket_val as u64;
        }
    }

    mul
}

#[test]
fn test_part2() {
    let s = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";
    let info = parse(s);

    assert_eq!(part2(&info), 1);
}

trait Bits {
    fn first_bit_set(self) -> Option<u8>;
    fn has_single_bit(self) -> bool;
}

impl Bits for u32 {
    fn first_bit_set(self) -> Option<u8> {
        for i in 0..32 {
            if (self & (1 << i)) != 0 {
                return Some(i);
            }
        }
        None
    }

    fn has_single_bit(self) -> bool {
        self != 0 && self & (self - 1) == 0
    }
}

fn part1(info: &Info) -> u32 {
    info
        .nearby_tickets
        .iter()
        .filter_map(|ticket| ticket.first_invalid_field(&info.field_rules))
        .sum()
}

#[test]
fn test_part1() {
    let s = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
    let info = parse(s);

    assert_eq!(part1(&info), 71);
}

struct Info {
    field_rules: Vec<FieldRule>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

trait FieldRulesExt {
    fn has_matching(&self, v: u32) -> bool;
}
impl FieldRulesExt for Vec<FieldRule> {
    fn has_matching(&self, v: u32) -> bool {
        self
            .iter()
            .map(|FieldRule { rule, .. }| rule)
            .filter(|rule| rule.matches(v))
            .next()
            .is_some()
    }
}

#[allow(dead_code)]
struct FieldRule {
    field: String,
    rule: Rule,
}

impl std::fmt::Debug for FieldRule {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "field \"{}\": {:?}", self.field, self.rule)
    }
}

#[derive(Debug)]
struct Rule {
    range_a: ops::RangeInclusive<u32>,
    range_b: ops::RangeInclusive<u32>,
}

impl Rule {
    fn matches(&self, v: u32) -> bool {
        let (a, b) = (&self.range_a, &self.range_b);
        a.contains(&v) || b.contains(&v)
    }
}

#[derive(Debug)]
struct Ticket {
    values: Vec<u32>,
}

impl Ticket {
    fn first_invalid_field(&self, field_rules: &Vec<FieldRule>) -> Option<u32> {
        let invalid = self
            .values
            .iter()
            .cloned()
            .find(|&v| !field_rules.has_matching(v));

        invalid
    }

    fn is_valid(&self, field_rules: &Vec<FieldRule>) -> bool {
        self.first_invalid_field(field_rules).is_none()
    }
}

fn parse(input: &str) -> Info {
    let mut info = Info {
        field_rules: Vec::new(),
        my_ticket: Ticket { values: Vec::new() },
        nearby_tickets: Vec::new(),
    };

    let mut iter = input.lines();

    let rule_re = Regex::new(r"^(.*): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();

    for line in &mut iter {
        if line.is_empty() {
            break;
        }

        let captures = match rule_re.captures(line) {
            Some(c) => c,
            None => {
                panic!("couldn't match {}", line);
            },
        };
        let capture = |i| captures.get(i).unwrap().as_str();
        let capture_to_n = |i| {
            let cap = capture(i);
            match cap.parse() {
                Ok(n) => n,
                Err(x) => {
                    panic!("couldn't parse {}: {}", cap, x);
                },
            }
        };

        let fr = FieldRule {
            field: capture(1).to_string(),
            rule: Rule {
                range_a: capture_to_n(2)..=capture_to_n(3),
                range_b: capture_to_n(4)..=capture_to_n(5),
            },
        };

        info.field_rules.push(fr);
    }

    assert_eq!(iter.next().unwrap(), "your ticket:");
    info.my_ticket = parse_ticket(iter.next().unwrap());

    assert_eq!(iter.next().unwrap(), "");
    assert_eq!(iter.next().unwrap(), "nearby tickets:");
    for line in iter {
        info.nearby_tickets.push(parse_ticket(line));
    }

    info
}

fn parse_ticket(s: &str) -> Ticket {
    Ticket {
        values: s.split(',')
            .map(|s| {
                match s.parse() {
                    Ok(x) => x,
                    Err(e) => {
                        panic!("couldn't parse '{}': {}", s, e);
                    },
                }
            })
            .collect(),
    }
}
