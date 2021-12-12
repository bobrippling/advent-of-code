use std::{
    collections::{HashMap, HashSet},
    convert::Infallible,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let caves = std::fs::read_to_string("input.txt")?.parse()?;

    println!("Part 1: {}", part1(&caves));
    println!("Part 2: {}", part2(&caves));

    Ok(())
}

fn part1(caves: &Caves) -> u64 {
    struct Rule;

    impl VisitRule for Rule {
        fn can_visit(&self, path: &Path, candidate: &Cave) -> bool {
            candidate.is_big() || path.find(candidate).next().is_none()
        }
    }

    caves.path_count(&Rule)
}

fn part2(caves: &Caves) -> u64 {
    struct Rule;

    impl VisitRule for Rule {
        fn can_visit(&self, path: &Path, candidate: &Cave) -> bool {
            if candidate.is_big() {
                return true;
            }

            if candidate.is_start() {
                return false;
            }

            match path.find(candidate).count() {
                0 => true,
                2 => false,
                1 => !path.has_double_visit(),
                _ => panic!("logic bug"),
            }
        }
    }

    caves.path_count(&Rule)
}

trait VisitRule {
    fn can_visit(&self, path: &Path, candidate: &Cave) -> bool;
}

struct Caves {
    edges: HashMap<Cave, HashSet<Cave>>,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Cave(String);

#[derive(Clone)]
struct Path<'cave> {
    steps: Vec<&'cave Cave>,
}

impl Caves {
    fn path_count(&self, visit_rule: &dyn VisitRule) -> u64 {
        let at: Cave = "start".parse().unwrap();

        let paths = self.path_trace(&at, &Path::new(&at), visit_rule);

        paths.len() as u64
    }

    fn path_trace<'a>(
        &'a self,
        at: &Cave,
        so_far: &Path<'a>,
        visit_rule: &dyn VisitRule,
    ) -> Vec<Path<'a>> {
        if at.is_end() {
            return vec![so_far.clone()];
        }

        let options = self.edges.get(at).unwrap();
        let mut paths = vec![];

        for option in options {
            if visit_rule.can_visit(so_far, option) {
                let next = so_far.appended(option);
                let tails = self.path_trace(option, &next, visit_rule);
                paths.extend(tails.into_iter());
            }
        }

        paths
    }
}

impl<'cave> Path<'cave> {
    fn new(start: &'cave Cave) -> Self {
        Self { steps: vec![start] }
    }

    fn find<'call>(&'call self, cave: &'call Cave) -> impl Iterator<Item = &'cave Cave> + 'call {
        self.steps.iter().copied().filter(move |&c| c == cave)
    }

    fn has_double_visit(&self) -> bool {
        let mut caves = HashSet::new();

        for &cave in self.steps.iter().filter(|c| !c.is_big()) {
            if caves.contains(cave) {
                return true;
            }
            caves.insert(cave);
        }

        false
    }

    fn append(&mut self, with: &'cave Cave) {
        self.steps.push(with);
    }

    fn appended(&self, with: &'cave Cave) -> Self {
        let mut c = self.clone();
        c.append(with);
        c
    }
}

impl Cave {
    fn is_start(&self) -> bool {
        self.0 == "start"
    }

    fn is_end(&self) -> bool {
        self.0 == "end"
    }

    fn is_big(&self) -> bool {
        self.0.chars().next().unwrap().is_ascii_uppercase()
    }
}

impl std::str::FromStr for Caves {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut edges = HashMap::<_, HashSet<_>>::new();

        for l in s.lines() {
            let parts = l.trim().split('-').collect::<Vec<_>>();

            if let [a, b] = parts[..] {
                let a: Cave = a.parse().unwrap();
                let b: Cave = b.parse().unwrap();

                let to = edges.entry(a.clone()).or_default();
                to.insert(b.clone());

                let to = edges.entry(b).or_default();
                to.insert(a);
            } else {
                panic!("couldn't split '{}'", l);
            }
        }

        Ok(Self { edges })
    }
}

impl std::str::FromStr for Cave {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Cave(s.into()))
    }
}

impl std::fmt::Debug for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Debug for Path<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, cave) in self.steps.iter().enumerate() {
            write!(
                f,
                "{:?}{}",
                cave,
                if i == self.steps.len() - 1 { "" } else { "," }
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static EG1: &'static str = "\
    start-A
    start-b
    A-c
    A-b
    b-d
    A-end
    b-end\
    ";

    static EG2: &'static str = "\
    dc-end
    HN-start
    start-kj
    dc-start
    dc-HN
    LN-dc
    HN-end
    kj-sa
    kj-HN
    kj-dc\
    ";

    static EG3: &'static str = "\
    fs-end
    he-DX
    fs-he
    start-DX
    pj-DX
    end-zg
    zg-sl
    zg-pj
    pj-he
    RW-he
    fs-DX
    pj-RW
    zg-RW
    start-pj
    he-WI
    zg-he
    pj-fs
    start-RW\
    ";

    #[test]
    fn test_part1() {
        for (eg, n) in [(EG1, 10), (EG2, 19), (EG3, 226)] {
            let caves = eg.parse().unwrap();
            assert_eq!(part1(&caves), n);
        }
    }

    #[test]
    fn test_part2() {
        for (eg, n) in [(EG1, 36), (EG2, 103), (EG3, 3509)] {
            let caves = eg.parse().unwrap();
            assert_eq!(part2(&caves), n);
        }
    }
}
