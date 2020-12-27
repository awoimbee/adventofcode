use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input/day21.txt");

struct Food {
    pub ingredients: Vec<&'static str>,
    pub allergens: Vec<&'static str>,
}

impl Food {
    pub fn new(ingredients: Vec<&'static str>, allergens: Vec<&'static str>) -> Self {
        Self {
            ingredients,
            allergens,
        }
    }
}

fn parse(input: &'static str) -> Vec<Food> {
    input
        .lines()
        .map(|food| {
            let sep = food.find(" (contains ").unwrap();
            let ingredients = food[..sep].split(' ').collect();
            let allergens = food[sep + 11..food.len() - 1].split(", ").collect();
            Food::new(ingredients, allergens)
        })
        .collect()
}

fn allergen_to_maybe_ingredients(foods: &[Food]) -> HashMap<&str, Vec<&str>> {
    let mut allergenics_ingredients = HashMap::new();
    for f in foods {
        for allergen in f.allergens.iter() {
            allergenics_ingredients
                .entry(*allergen)
                .and_modify(|sus_ingredients: &mut Vec<&str>| {
                    sus_ingredients.retain(|ing| f.ingredients.contains(&ing));
                })
                .or_insert(f.ingredients.clone());
        }
    }
    allergenics_ingredients
}

fn allergens_and_inert_ingredients(foods: &[Food]) -> (HashMap<&str, Vec<&str>>, Vec<&str>) {
    let mut allergen_ingredients = allergen_to_maybe_ingredients(&foods);
    let maybe_allergenic_ingredients: HashSet<_> = allergen_ingredients
        .iter()
        .map(|(_allergen, ingredients)| ingredients.into_iter())
        .flatten()
        .collect();
    let innert_ingredients: Vec<&str> = foods
        .iter()
        .map(|f| f.ingredients.iter())
        .flatten()
        .filter(|ing| !maybe_allergenic_ingredients.contains(*ing))
        .map(|i| *i)
        .collect();
    allergen_ingredients.iter_mut().for_each(|(_al, ings)| {
        ings.retain(|ing| !innert_ingredients.contains(ing));
    });
    (allergen_ingredients, innert_ingredients)
}

fn part1(foods: &[Food], innert: Vec<&str>) -> u64 {
    let apparition_nb: u64 = foods
        .iter()
        .map(|f| {
            f.ingredients
                .iter()
                .map(|ing| (innert.contains(ing)) as u64)
                .sum::<u64>()
        })
        .sum();
    apparition_nb
}

fn part2(mut allergens: HashMap<&str, Vec<&str>>) -> String {
    // allergens.into_iter().sorted().flat_map(|(_, ingredients)| {
    //     ingredients.into_iter().sorted()
    // }).collect::<BTreeSet<_>>().into_iter().join(",")
    let mut last_used = "";
    let mut new_used = "";
    let mut allergen_ingredient: Vec<(&str, &str)> = Vec::new();

    while !allergens.is_empty() {
        let mut allergen_to_remove = "";
        for (a, ings) in allergens.iter_mut() {
            ings.retain(|i| *i != last_used);
            if ings.len() == 1 && new_used == "" {
                new_used = ings[0];
                allergen_ingredient.push((*a, ings[0]));
                allergen_to_remove = a;
            }
        }
        allergens.remove(allergen_to_remove);
        last_used = new_used;
        new_used = "";
    }
    allergen_ingredient.sort_by(|a, b| a.0.cmp(b.0));
    allergen_ingredient.into_iter().map(|(_a, i)| i).join(",")
}

pub fn day21() -> (String, String) {
    let foods = parse(INPUT);
    let (allergens, innert_ingredients) = allergens_and_inert_ingredients(&foods);

    let p1 = part1(&foods, innert_ingredients);
    let p2 = part2(allergens);

    (p1.to_string(), p2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = concat!(
        "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\n",
        "trh fvjkl sbzzf mxmxvkd (contains dairy)\n",
        "sqjhc fvjkl (contains soy)\n",
        "sqjhc mxmxvkd sbzzf (contains fish)\n"
    );

    #[test]
    fn test_p1() {
        let parsed = parse(TEST_INPUT);
        let (_, innert_ingredients) = allergens_and_inert_ingredients(&parsed);
        assert!(part1(&parsed, innert_ingredients) == 5);
    }

    #[test]
    fn test_p2() {
        let parsed = parse(TEST_INPUT);
        let (allergens, _) = allergens_and_inert_ingredients(&parsed);
        assert!(part2(allergens) == "mxmxvkd,sqjhc,fvjkl");
    }
}
