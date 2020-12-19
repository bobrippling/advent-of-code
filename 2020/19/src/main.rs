use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Write;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let (mut rules, msgs) = parse(&input);

    println!("Part 1: {}", count_matching(&rules, &msgs));

    part2_edit(&mut rules);
    println!("Part 2: {}", count_matching(&rules, &msgs));
}

#[derive(Debug)]
enum Rule {
    Char(char),
    Concat(Vec<usize>),
    Or(Vec<Box<Rule>>),

    // Part 2
    RegexPlus(usize),
    OptionalMiddle(usize, usize, usize),
}

struct Message<'a>(&'a str);

fn parse<'a>(s: &'a str) -> (HashMap<usize, Rule>, Vec<Message<'a>>) {
    let mut iter = s.lines();

    let mut rules = HashMap::new();
    for line in &mut iter {
        if line.is_empty() {
            break;
        }

        let mut parts = line.split(": ");

        let index: usize = parts.next().unwrap().parse().unwrap();

        let rhs = parts.next().unwrap();
        let mut rhs_chars = rhs.chars().peekable();

        if rhs_chars.peek().unwrap() == &'"' {
            rhs_chars.next().unwrap();
            rules.insert(index, Rule::Char(rhs_chars.next().unwrap()));
            assert_eq!(rhs_chars.next().unwrap(), '"');
            assert_eq!(rhs_chars.next(), None);
        } else {
            let subrules_str = rhs.split(" | ");
            assert!(parts.next().is_none());

            let mut subrules_v = vec![];
            for subrule in subrules_str {
                let indicies = subrule
                    .split(' ')
                    .map(str::parse)
                    .map(Result::unwrap)
                    .collect::<Vec<_>>();

                subrules_v.push(Rule::Concat(indicies));
            }

            if subrules_v.len() == 1 {
                rules.insert(index, subrules_v.pop().unwrap());
            } else {
                rules.insert(index, Rule::Or(subrules_v.into_iter().map(Box::new).collect()));
            }
        }
    }

    let messages = iter.map(Message).collect();

    (rules, messages)
}

fn count_matching(rules: &HashMap<usize, Rule>, msgs: &Vec<Message>) -> usize {
    msgs
        .iter()
        .filter(|msg| {
            full_match(rules, msg.0)
        })
        .count()
}

fn full_match(rules: &HashMap<usize, Rule>, msg: &str ) -> bool {
    let rule0 = rules.get(&0).unwrap();

    rule0.matches(msg, rules, 0).contains(&msg.len())
}

// #[allow(dead_code)]
// fn count_matching_via_re(rules: &HashMap<usize, Rule>, msgs: &Vec<Message>) -> usize {
//     let mut s = String::new();
//     rules[&0].as_re(rules, &mut s, 0);

//     let s = "^".to_string() + &s + "$";
//     let re = Regex::new(&s).unwrap();

//     println!("re: '{}'", re);

//     msgs
//         .iter()
//         .filter(|msg| {
//             re.is_match(msg.0)
//         })
//         .count()
// }

#[test]
fn test_count_matching() {
    let eg = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;

    // ababbb & abbbab

    let (rules, msgs) = parse(&eg);
    assert_eq!(count_matching(&rules, &msgs), 2);
}

fn part2_edit(rules: &mut HashMap<usize, Rule>) {
    // 31: 13 3 | 92 26
    // 42: 13 127 | 92 67
    //
    // 3: 57 92 | 117 13
    // 127: 91 13 | 1 92
    // 92: "a"
    // 26: 46 13 | 97 92
    // 13: "b"
    // 67: 81 92 | 10 13
    //
    // 57: 66 92 | 40 13
    //
    // 117: 132 92 | 109 13
    // 46: 92 83 | 13 122
    // 57: 66 92 | 40 13
    // 97: 89 92 | 62 13
    // 10: 61 13 | 94 92
    // 1: 88 92 | 126 13
    // 13: "b"
    // 81: 13 51 | 92 114
    // 91: 92 18 | 13 101

    *rules.get_mut(&8).unwrap() =
        Rule::RegexPlus(42);

    *rules.get_mut(&11).unwrap() =
        Rule::OptionalMiddle(42, 11, 31);
}

#[test]
fn test_count_matching2() {
    let eg = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;

    let (mut rules, msgs) = parse(&eg);
    part2_edit(&mut rules);

    let match_eg = "babbbbaabbbbbabbbbbbaabaaabaaa";
    assert!(full_match(&rules, match_eg));

    assert_eq!(count_matching(&rules, &msgs), 12);
}

impl Rule {
    // returns length of match
    // returns possible lengths of submatches
    fn matches(&self, msg: &str, rules: &HashMap<usize, Rule>, depth: usize) -> HashSet<usize> {
        let debug = false;
        let indent = || for _ in 0..depth {
            print!("  ");
        };
        if debug {
            indent();
            println!("trying {:?} on {}", self, msg);
        }

        let lengths = match self {
            Rule::Char(ch) => {
                let mut s = HashSet::new();

                match msg.chars().next() {
                    Some(mch) if *ch == mch => {
                        s.insert(1);
                    }
                    _ => {}
                };

                s
            },
            Rule::Concat(indicies) => {
                let mut sublengths = HashSet::new();
                sublengths.insert(0);

                for rule_i in indicies {
                    let rule = &rules[rule_i];

                    let mut new_sublengths = HashSet::new();

                    for &sublen in &sublengths {
                        rule
                            .matches(&msg[sublen..], rules, depth + 1)
                            .iter()
                            .map(|l| l + sublen)
                            .for_each(|l| {
                                new_sublengths.insert(l);
                            });
                    }
                    sublengths = new_sublengths;
                }
                sublengths
            }
            Rule::Or(subrules) => {
                subrules
                    .iter()
                    .flat_map(|rule| {
                        rule.matches(msg, rules, depth + 1)
                    })
                    .collect::<HashSet<_>>()
            }

            Rule::RegexPlus(subrule) => {
                let subrule = &rules[subrule];
                let mut sublengths = HashSet::new();
                sublengths.insert(0);

                if debug {
                    indent();
                    println!("RegexPlus subrule {:?}", subrule);
                    // let mut s = String::new();
                    // self.as_re(rules, &mut s, 0);
                    // println!("RegexPlus {}", s);
                }

                loop {
                    let mut to_append = HashSet::new();
                    for &sublen in &sublengths {
                        let matches = subrule.matches(&msg[sublen..], rules, depth + 1);
                        if debug {
                            indent();
                            println!("RegexPlus submatches from {}: {:?}", sublen, matches);
                        }
                        for m in matches {
                            to_append.insert(sublen + m);
                        }
                    }
                    let len = sublengths.len();
                    sublengths.extend(to_append);
                    if len == sublengths.len() {
                        break;
                    }
                }
                sublengths.remove(&0);
                sublengths
            }

            Rule::OptionalMiddle(l, m, r) => {
                let (l, m, r) = (&rules[l], &rules[m], &rules[r]);
                let l_matches = l.matches(msg, rules, depth + 1);

                if debug {
                    indent();
                    println!("OptMid left: {:?}", l_matches);
                }

                let submatches =
                    l_matches
                        .iter()
                        .flat_map(|&lmatch| {
                            let with = m.matches(&msg[lmatch..], rules, depth + 1);
                            with
                                .into_iter()
                                .map(move |w| lmatch + w)
                        })
                    .collect::<HashSet<_>>();

                if debug {
                    indent();
                    println!("OptMid mid: {:?}", submatches);
                }

                let mut matches = l_matches;
                matches.extend(submatches);
                let matches = matches;

                let mut new_sublengths = HashSet::new();
                for sublen in matches {
                    r
                        .matches(&msg[sublen..], rules, depth + 1)
                        .iter()
                        .map(|l| l + sublen)
                        .for_each(|l| {
                            new_sublengths.insert(l);
                        });
                }
                new_sublengths
            }
        };

        if debug {
            indent();
            println!("got {:?}", lengths);
        }

        lengths
    }

    #[allow(dead_code)]
    fn as_re(&self, rules: &HashMap<usize, Rule>, out: &mut String, depth: usize) {
        match self {
            Rule::Char(ch) => {
                write!(out, "{}", ch).unwrap();
            }
            Rule::Concat(indicies) => {
                indicies
                    .iter()
                    .map(|i| &rules[i])
                    .for_each(|r| {
                        r.as_re(rules, out, depth + 1);
                    });
            }
            Rule::Or(subrules) => {
                write!(out, "(").unwrap();

                let or = subrules
                    .iter()
                    .map(|br| &*br)
                    .map(|r| {
                        let mut s = String::new();
                        r.as_re(rules, &mut s, depth + 1);
                        s
                    });

                let mut sep = "";
                for ent in or {
                    write!(out, "{}{}", sep, ent).unwrap();
                    sep = "|";
                }

                write!(out, ")").unwrap();
            }

            Rule::RegexPlus(subrule) => {
                write!(out, "(").unwrap();
                rules[subrule].as_re(rules, out, depth + 1);
                write!(out, ")+").unwrap();
            }

            Rule::OptionalMiddle(l, m, r) => {
                rules[l].as_re(rules, out, depth + 1);

                write!(out, "(").unwrap();
                rules[m].as_re(rules, out, depth + 1);
                write!(out, ")?").unwrap();

                rules[r].as_re(rules, out, depth + 1);
            }
        };
    }
}
