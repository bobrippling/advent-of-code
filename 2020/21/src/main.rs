use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let s = fs::read_to_string("./input.txt").unwrap();

    let mix = parse(&s);
    println!("Part 1: {}", part1(&mix));

    println!("Part 2: {}", part2(&mix));
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Ingredient(String);

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Allergen(String);

struct Mix {
    // FIXME: split these out and use references from map
    map: Vec<(HashSet<Ingredient>, HashSet<Allergen>)>,
    ingredients: HashSet<Ingredient>,
    allergens: HashSet<Allergen>,
}

fn parse(s: &str) -> Mix {
    let mut map = Vec::<(HashSet<Ingredient>, HashSet<Allergen>)>::new();
    // FIXME: use references below
    let mut all_allergens = HashSet::<Allergen>::new();
    let mut all_ingredients = HashSet::<Ingredient>::new();

    s.lines()
        .for_each(|mut l| {
            assert!(l.ends_with(")"));
            l = &l[..=l.len()-2];
            let mut parts = l.split(" (contains ");
            let ingredients = parts.next().unwrap();
            let allergens = parts.next().unwrap();
            assert!(parts.next().is_none());

            let ingredients = ingredients.replace(',', "");
            let allergens = allergens.replace(',', "");

            let ingredients = ingredients.split(' ').map(str::to_string).map(Ingredient).collect::<HashSet<Ingredient>>();
            let allergens = allergens.split(' ').map(str::to_string).map(Allergen).collect::<HashSet<Allergen>>();

            all_allergens.extend(allergens.clone());
            all_ingredients.extend(ingredients.clone());

            map.push((
                ingredients,
                allergens,
            ));
        });
    Mix {
        map,
        allergens: all_allergens,
        ingredients: all_ingredients,
    }
}

#[test]
fn test_part1() {
    // ingredient... (contains allergen...)
    // none of kfcds, nhms, sbzzf, or trh can contain an allergen.
    let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

    let mix = parse(&input);
    // println!("mix: {:?}", mix);
    // mxmxvkd: (dairy, fish), (dairy)

    assert_eq!(part1(&mix), 5);
}

fn get_safe_and_risky(mix: &Mix) -> (HashSet<Ingredient>, HashSet<Ingredient>) {
    let mut risky_ingredients = HashSet::<&Ingredient>::new();

    for allergen in &mix.allergens {
        let mut possibilities = HashSet::<&Ingredient>::new();

        for (ingredients, allergens) in &mix.map {
            if allergens.contains(&allergen) {
                if possibilities.is_empty() {
                    possibilities.extend(ingredients);
                } else {
                    let addrs: HashSet<&Ingredient> = ingredients.iter().map(|x| x).collect();
                    possibilities = possibilities.intersection(&addrs).cloned().collect();
                }
            }
        }

        // println!("{:?} must be in one of: {:?}", allergen, possibilities);
        risky_ingredients.extend(possibilities.clone());
    }

    let risky_ingredients = risky_ingredients.iter().cloned().cloned().collect::<HashSet<Ingredient>>();
    let safe_ingredients = mix.ingredients.difference(&risky_ingredients);

    let safe_ingredients: HashSet<&Ingredient> = safe_ingredients.collect();
    let safe_ingredients: HashSet<Ingredient> = safe_ingredients.iter().cloned().cloned().collect::<HashSet<Ingredient>>();

    (safe_ingredients, risky_ingredients)
}

fn part1(mix: &Mix) -> usize {
    let (safe_ingredients, _risky_ingredients) = get_safe_and_risky(mix);

    // println!("safe: {:?}", safe_ingredients);

    mix.map
        .iter()
        .map(|(ingredients, _)| {
            ingredients.intersection(&safe_ingredients).count()
        })
        .sum()
}

fn part2(mix: &Mix) -> String {
    let mut allergen_to_ingredients = HashMap::<Allergen, HashSet<Ingredient>>::new();

    for allergen in &mix.allergens {
        let mut possibilities = HashSet::<&Ingredient>::new();

        for (ingredients, allergens) in &mix.map {
            if allergens.contains(&allergen) {
                if possibilities.is_empty() {
                    possibilities.extend(ingredients);
                } else {
                    let addrs: HashSet<&Ingredient> = ingredients.iter().map(|x| x).collect();
                    possibilities = possibilities.intersection(&addrs).cloned().collect();
                }
            }
        }

        // println!("{:?} must be in one of: {:?}", allergen, possibilities);
        let possibilities = possibilities.iter().cloned().cloned().collect();
        allergen_to_ingredients.insert(allergen.clone(), possibilities);
    }

    let mut allergen_to_ingredient = HashMap::<Allergen, Ingredient>::new();

    loop {
        let mut ingredient = Option::<Ingredient>::None;
        for (allergen, possibs) in &allergen_to_ingredients {
            if possibs.len() == 1 {
                let i = possibs.iter().next().unwrap();
                ingredient = Some(i.clone());
                allergen_to_ingredient.insert(allergen.clone(), i.clone());
                // println!("{:?} contains {:?}", i, allergen);
                break;
            }
        }

        if let Some(i) = ingredient {
            // println!("dropping {:?}...", i);
            for (_, ingredients) in allergen_to_ingredients.iter_mut() {
                ingredients.remove(&i);
            }
        } else {
            break;
        }
    }

    let mut allergen_alpha = allergen_to_ingredient.keys().collect::<Vec<_>>();
    allergen_alpha.sort();

    let mut s = String::new();

    allergen_alpha
        .iter()
        .map(|a| &allergen_to_ingredient[a])
        .for_each(|i| {
            s.push_str(&i.0);
            s.push(',');
        });

    s.pop();
    s
}

#[test]
fn test_part2() {
    let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

    let mix = parse(&input);
    // println!("mix: {:?}", mix);
    // mxmxvkd: (dairy, fish), (dairy)

    assert_eq!(part2(&mix), "mxmxvkd,sqjhc,fvjkl");
}
