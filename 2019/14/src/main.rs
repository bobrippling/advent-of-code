use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Element(String);

impl std::fmt::Debug for Element {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.0)
    }
}

type Ingredient = (Element, usize);

struct Reactions {
    //src2dst: HashMap<Vec<Ingredient>, Ingredient>,
    dst2src: HashMap<Element, (usize, Vec<Ingredient>)>,
}

/*
fn printindent(n: usize) {
    for _ in 0..n {
        print!("  ");
    }
}
*/

fn ingredient_from_str(s: &str) -> Ingredient {
    let parts = s.split(' ').filter(|s| !s.is_empty()).collect::<Vec<_>>();

    if parts.len() != 2 {
        eprintln!("parse ingredient_from_str: {:?}", parts);
    }


    assert_eq!(parts.len(), 2);
    (
        Element(String::from(parts[1])),
        parts[0].parse().expect("parseint"),
    )
}

impl Element {
    fn new(s: &str) -> Self {
        Element(s.into())
    }
}

impl Reactions {
    fn from(s: &str) -> Self {
        let list = s
            .split('\n')
            .filter(|x| !x.trim().is_empty())
            .map(|line| {
                let parts = line.split(" => ").collect::<Vec<_>>();

                if parts.len() != 2 {
                    eprintln!("parse sides: {:?}", parts);
                }

                assert_eq!(parts.len(), 2); // .expect("no \"=>\" in {}", line);

                let from = parts[0].split(", ");
                let to = parts[1];

                let from_ingredients = from.map(ingredient_from_str).collect::<Vec<_>>();

                (from_ingredients, ingredient_from_str(to))
            });

        //let mut src2dst = HashMap::new();
        let mut dst2src = HashMap::new();
        for (from_ingredients, to) in list {
            //let prev = src2dst.insert(from_ingredients, to);
            //assert!(prev.is_none());

            let prev = dst2src.insert(
                to.0,
                (
                    to.1,
                    from_ingredients,
                )
            );
            assert!(prev.is_none());
        }

        Reactions { dst2src /*, src2dst*/ }
    }

    fn get(&self, key: &Element) -> Option<&(usize, Vec<Ingredient>)> {
        self.dst2src.get(key)
    }

    /*
    fn work_backwards_one(&self, req: &Ingredient, usize>, indent: usize) {
        if self.take(req) {
        }

        let &(req_gained, ingredients) = match self.get(&req.0) {
            Some(ref x) => {
                printindent(indent);
                println!("{:?} --> {:?}", req.0, x);
                x
            },
            None => {
                // can't get it, must be raw, so we need 'req' many of it
                raw.insert(
                    req.0,
                    raw.get(&req.0).map(|x| *x).unwrap_or(0) + req.1);
                printindent(indent);
                println!("{:?} --> NOTHING, returning {}", req, req.1);
                return req.1;
            }
        };

        let mul = 1;
        while req_gained * mul < req.1 {
            mul += 1;
        }

        let t = mul * req_gained * ingredients.iter().fold(
            0,
            |total, (elem, n)| {
                total + self.work_backwards_one(
                    &(elem.clone(), *n),
                    raw,
                    indent + 1)
            });
        printindent(indent);
        println!("returning {} for {:?}", t, req);
        t
    }
    */

    fn iterate(&self, req: &Ingredient) -> HashMap<Element, usize> {
        let mut tobuild = HashMap::<Element, usize>::new();
        //let mut roots = HashMap::<Element, usize>::new();

        tobuild.insert(req.0.clone(), req.1);

        while tobuild.len() > 0 {
            println!("tobuild: {:?}", tobuild);

            let mut candidates = tobuild
                .iter()
                .filter(|(ref elem, _)| {
                    self.get(&elem).is_some()
                })
                .collect::<Vec<_>>();

            if candidates.len() == 0 {
                break;
            }

            // find the one with the least n
            candidates.sort_by(
                |a, b| {
                    a.1.cmp(b.1)
                });

            let first = candidates[0];

            let (ref selected, ref nreq) = first;

            let (selected, nreq): (Element, usize) = ((*selected).clone(), **nreq);

            println!("  picked {:?} to build (need {})", selected, nreq);

            let &(nmade, ref ingredients) = self.get(&selected).unwrap(); /* {
                Some(x) => x,
                None => {
                    println!("  is a root, added");
                    assert_eq!(selected.0, "ORE");
                    let already = roots.get(&selected).map(|x| *x).unwrap_or(0);
                    roots.insert(selected.clone(), already + nreq);
                    continue;
                }
            };*/

            let mut mul = 1;
            while nmade * mul < nreq {
                mul += 1;
            }

            println!("  using recipe {:?} --> {} to make {} {:?} (need {} mul as nreq={})",
                     ingredients, nmade,
                     nmade * mul, selected,
                     mul, nreq);

            if nreq < nmade {
                // we've made plenty, remove
                tobuild.remove(&selected);
            } else {
                let left = nreq - nmade;
                if left > 0 {
                    tobuild.insert(selected.clone(), left);
                } else {
                    tobuild.remove(&selected);
                }
            }

            for (ref elem, n) in ingredients {
                /*
                match tobuild.get(&elem) {
                    None => {},
                    Some(&already_have) => {
                        let remaining;
                        let to_use;

                        if already_have > nreq {
                            to_use = nreq;
                            remaining = already_have - to_use;
                        } else {
                            to_use = already_have;
                            remaining = 0;
                        }

                        println!("  already have {} {:?}s, using {}, leaving {} in tobuild",
                                 already_have,
                                 selected,
                                 to_use,
                                 remaining);

                        tobuild.insert(selected.clone(), remaining);

                        nreq -= to_use;
                    },
                };
                */

                let already_tobuild = tobuild.get(elem).map(|x| *x).unwrap_or(0);

                tobuild.insert(elem.clone(), already_tobuild + n /* * mul*/);
            }
        }

        tobuild
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let reactions = "
            9 ORE => 2 A
            8 ORE => 3 B
            7 ORE => 5 C
            3 A, 4 B => 1 AB
            5 B, 7 C => 1 BC
            4 C, 1 A => 1 CA
            2 AB, 3 BC, 4 CA => 1 FUEL
        ";

        let reactions = Reactions::from(reactions);

        assert_eq!(reactions.dst2src.len(), 7);
        /*
        assert_eq!(reactions.get(
                &vec![((Element::new("ORE"), 9))]),
                Some(&(Element::new("A"), 2)));

        assert_eq!(
            reactions.get(&vec![(Element::new("ORE"), 8)]),
            Some(&(Element::new("B"), 3)));
        assert_eq!(
            reactions.get(
                &vec![
                    (Element::new("ORE"), 7)
                ]),
            Some(&(Element::new("C"), 5)));
        assert_eq!(
            reactions.get(
                &vec![
                    (Element::new("A"), 3),
                    (Element::new("B"), 4)
                ]),
          Some(&(Element::new("AB"), 1)));
        assert_eq!(
            reactions.get(
                &vec![
                    (Element::new("B"), 5),
                    (Element::new("C"), 7)
                ]),
                Some(&(Element::new("BC"), 1)));
        assert_eq!(
            reactions.get(
                &vec![
                    (Element::new("C"), 4),
                    (Element::new("A"), 1)
                ]),
                Some(&(Element::new("CA"), 1)));

        assert_eq!(
            reactions.get(
                &vec![
                    (Element::new("AB"), 2),
                    (Element::new("BC"), 3),
                    (Element::new("CA"), 4)
                ]),
            Some(&(Element::new("FUEL"), 1)));
        */

            assert_eq!(reactions.get(&Element::new("A")), Some(&(2, vec![(Element::new("ORE"), 9)])));
            assert_eq!(reactions.get(&Element::new("B")), Some(&(3, vec![(Element::new("ORE"), 8)])));
            assert_eq!(reactions.get(&Element::new("C")), Some(&(5, vec![(Element::new("ORE"), 7)])));
            assert_eq!(reactions.get(&Element::new("AB")), Some(&(1, vec![(Element::new("A"), 3), (Element::new("B"), 4)])));
            assert_eq!(reactions.get(&Element::new("BC")), Some(&(1, vec![(Element::new("B"), 5), (Element::new("C"), 7)])));
            assert_eq!(reactions.get(&Element::new("CA")), Some(&(1, vec![(Element::new("C"), 4), (Element::new("A"), 1)])));
            assert_eq!(reactions.get(&Element::new("FUEL")), Some(&(1, vec![(Element::new("AB"), 2), (Element::new("BC"), 3), (Element::new("CA"), 4)])));
    }

    #[test]
    fn eg1() {
        let reactions = "
            9 ORE => 2 A
            8 ORE => 3 B
            7 ORE => 5 C
            3 A, 4 B => 1 AB
            5 B, 7 C => 1 BC
            4 C, 1 A => 1 CA
            2 AB, 3 BC, 4 CA => 1 FUEL
        ";

        let reactions = Reactions::from(reactions);

        let roots = reactions.iterate(&(Element::new("FUEL"), 1));

        assert_eq!(roots.len(), 1);

        let first = roots.iter().nth(0).unwrap();
        assert_eq!(first, (&Element::new("ORE"), &165));
    }

    #[test]
    fn eg2() {
        let reactions = "
    157 ORE => 5 NZVS
    165 ORE => 6 DCFZ
    44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
    12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
    179 ORE => 7 PSHF
    177 ORE => 5 HKGWZ
    7 DCFZ, 7 PSHF => 2 XJWVT
    165 ORE => 2 GPVTF
    3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
        ";

        let reactions = Reactions::from(reactions);

        let roots = reactions.iterate(&(Element::new("FUEL"), 1));

        assert_eq!(roots.len(), 1);

        let first = roots.iter().nth(0).unwrap();
        assert_eq!(first, (&Element::new("ORE"), &13312));
    }

    #[test]
    fn eg3() {
        let reactions = "
    2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
    17 NVRVD, 3 JNWZP => 8 VPVL
    53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
    22 VJHF, 37 MNCFX => 5 FWMGM
    139 ORE => 4 NVRVD
    144 ORE => 7 JNWZP
    5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
    5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
    145 ORE => 6 MNCFX
    1 NVRVD => 8 CXFTF
    1 VJHF, 6 MNCFX => 4 RFSQX
    176 ORE => 6 VJHF
        ";

        let reactions = Reactions::from(reactions);

        let roots = reactions.iterate(&(Element::new("FUEL"), 1));

        assert_eq!(roots.len(), 1);

        let first = roots.iter().nth(0).unwrap();
        assert_eq!(first, (&Element::new("ORE"), &180697));
    }

    #[test]
    fn eg4() {
        let reactions = "
    171 ORE => 8 CNZTR
    7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
    114 ORE => 4 BHXH
    14 VRPVC => 6 BMBT
    6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
    6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
    15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
    13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
    5 BMBT => 4 WPTQ
    189 ORE => 9 KTJDG
    1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
    12 VRPVC, 27 CNZTR => 2 XDBXC
    15 KTJDG, 12 BHXH => 5 XCVML
    3 BHXH, 2 VRPVC => 7 MZWV
    121 ORE => 7 VRPVC
    7 XCVML => 6 RJRHP
    5 BHXH, 4 VRPVC => 5 LTCX
        ";

        let reactions = Reactions::from(reactions);

        let roots = reactions.iterate(&(Element::new("FUEL"), 1));

        assert_eq!(roots.len(), 1);

        let first = roots.iter().nth(0).unwrap();
        assert_eq!(first, (&Element::new("ORE"), &2210736));
    }
}
