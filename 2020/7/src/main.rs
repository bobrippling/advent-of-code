use std::fs;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = fs::read_to_string("./input.txt")?;

    let rule_map = parse_rules(&s);

    let mine = "shiny gold";

    let n = outermost_containing(&rule_map, &mine);
    println!("Part 1: {}", n);

    let n = total_bags(&rule_map, &mine);
    println!("Part 2: {}", n);

    Ok(())
}

type RuleMap = HashMap<String, Vec<(usize, String)>>;

fn parse_rules(s: &str) -> RuleMap {
    let rules = s.split('\n')
        .filter(|line| !line.is_empty())
        .map(parse_rule)
        .collect::<Vec<_>>();
    let mut rule_map = HashMap::new();
    for rule in rules {
        rule_map
            .entry(rule.bag.to_string())
            .or_insert(rule.contents);
    }

    /*
    for (bag, contents) in &rule_map {
        println!("\"{}\" contains: {:?}", bag, contents);
    }
    */

    rule_map
}

fn total_bags(
    rules: &RuleMap,
    start: &str,
) -> usize {
    let mut work = vec![(start, 1)];
    let mut count = 0;

    while let Some((bag, n)) = work.pop() {
        //println!("found {} {}(s)", n, bag);

        count += n;

        rules.get(bag)
            .unwrap()
            .iter()
            .for_each(|(this_n, bag)| {
                /*println!(
                    "  adding {} to work, and {} * {} to count",
                    bag, this_n, n);*/

                work.push((bag, this_n * n));
            });
    }

    count - 1
}

#[test]
fn test_total_bags() {
    let rule_map = parse_rules(EG);

    let mine = "shiny gold";
    let n = total_bags(&rule_map, &mine);
    assert_eq!(n, 32);
}

fn outermost_containing(
    rules: &RuleMap,
    target: &str,
) -> usize {
    let mut count = 0;
    for (bag, _) in rules {
        if bag == target {
            continue;
        }

        let y = eventually_contains(bag, target, &rules);
        //println!("{} eventually contains {}: {}", bag, target, y);

        if y {
            count += 1;
        }
    }

    count
}

fn eventually_contains(bag: &str, target: &str, rules: &RuleMap) -> bool {
    let mut work = vec![bag];

    while let Some(bag) = work.pop() {
        if bag == target {
            return true;
        }

        if let Some(contents) = rules.get(bag) {
            contents.iter()
                .map(|(_, bag)| bag)
                .for_each(|bag| work.push(bag));
        } else {
            eprintln!("no entry found for {}", bag);
        }
    }

    false
}

#[cfg(test)]
static EG: &str = "\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

#[test]
fn test_count_containing() {
    // A bright white bag, which can hold your shiny gold bag directly.
    // A muted yellow bag, which can hold your shiny gold bag directly, plus some other bags.
    // A dark orange bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.
    // A light red bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.

    let rule_map = parse_rules(EG);

    let mine = "shiny gold";
    let n = outermost_containing(&rule_map, &mine);
    assert_eq!(n, 4);
}

#[derive(Eq, PartialEq, Debug)] //order
struct Rule {
    bag: String,
    contents: Vec<(usize, String)>,
}

fn parse_rule(s: &str) -> Rule {
    let mut parts = s.split(" contain ");

    let bag = parts.next().unwrap().to_string();
    let bag = bag.replace(" bags", "");

    let bags = parts.next().expect(&format!("no next part in {}", s)[..])
        .split(", ")
        .filter_map(|input| {
            let chars = input.char_indices();
            let mut last = 0;

            for (i, char) in chars {
                if !char.is_numeric() {
                    break;
                }
                last = i;
            }
            let contents = &input[..=last];

            let n = match contents.parse::<usize>() {
                Ok(n) => n,
                Err(e) => {
                    if input == "no other bags." {
                        return None;
                    } else {
                        panic!("Couldn't parse \"{}\": {} // in \"{}\"",
                               &input[..=last],
                               e,
                               input);
                    }
                },
            };
            let desc = input[last+2 /* assumes space */..].to_string();
            let desc = desc.replace(" bags", "");
            let desc = desc.replace(" bag", "");
            let desc = desc.replace(".", "");

            Some((n, desc))
        })
        .collect::<Vec<_>>();

    Rule {
        bag,
        contents: bags,
    }
}

#[test]
fn test_parse_rule() {
    let r1 = "dim black bags contain 4 vibrant turquoise bags.";
    assert_eq!(
        parse_rule(r1),
        Rule {
            bag: "dim black".into(),
            contents: vec![
                (4, "vibrant turquoise".into()),
            ],
        },
    );

    let r2 = "dotted gray bags contain 1 posh salmon bag, 5 drab lime bags, 1 clear coral bag, 1 faded lime bag.";
    assert_eq!(
        parse_rule(r2),
        Rule {
            bag: "dotted gray".into(),
            contents: vec![
                (1, "posh salmon".into()),
                (5, "drab lime".into()),
                (1, "clear coral".into()),
                (1, "faded lime".into()),
            ],
        },
    );
}
