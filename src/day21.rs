use crate::prelude::*;

day!(21, parse_and_process => pt1, pt2);

#[derive(Debug, Clone)]
pub struct IngredientList<'s> {
    ingredients: HashSet<&'s str>,
    allergens: HashSet<&'s str>,
}

pub fn pt1(
    (ingredient_lists, known_allergens): &(Vec<IngredientList>, Vec<(&str, &str)>),
) -> Result<usize> {
    let known_ingredients = known_allergens
        .iter()
        .map(|(_, ingredient)| *ingredient)
        .collect::<HashSet<&str>>();
    Ok(ingredient_lists
        .iter()
        .flat_map(|l| l.ingredients.iter())
        .count_if(|&ingredient| !known_ingredients.contains(ingredient)))
}

pub fn pt2((_, known_allergens): &(Vec<IngredientList>, Vec<(&str, &str)>)) -> String {
    known_allergens
        .iter()
        .sorted_by_key(|(allergen, _)| *allergen)
        .map(|(_, ingredient)| ingredient)
        .join(",")
}

pub fn parse(input: &str) -> Result<Vec<IngredientList>> {
    use framework::parser::*;
    let ingredients = map(separated_list1(char(' '), alpha1), |v| {
        v.into_iter().collect()
    });
    let allergens = preceded(
        tag(" (contains "),
        terminated(
            map(separated_list1(tag(", "), alpha1), |v| {
                v.into_iter().collect()
            }),
            tag(")"),
        ),
    );
    let ingredient_list = map(pair(ingredients, allergens), |(ingredients, allergens)| {
        IngredientList {
            ingredients,
            allergens,
        }
    });
    separated_list1(char('\n'), ingredient_list)(input).into_result()
}

pub fn parse_and_process(input: &str) -> Result<(Vec<IngredientList>, Vec<(&str, &str)>)> {
    let ingredient_lists = parse(input)?;

    let mut pending_allergens = HashMap::<&str, usize>::new();
    for &allergen in ingredient_lists.iter().flat_map(|l| l.allergens.iter()) {
        *pending_allergens.entry(allergen).or_default() += 1;
    }
    let mut pending_allergens = pending_allergens.into_iter().collect::<Vec<_>>();
    pending_allergens.sort_unstable_by_key(|(_, b)| *b);

    let mut known_allergens = Vec::<(&str, &str)>::new();

    let mut intersection = HashSet::<&str>::new();
    'outer: while !pending_allergens.is_empty() {
        for (pending_index, (allergen, _)) in pending_allergens.iter().cloned().enumerate().rev() {
            // Create an intersection of the ingredients of all recipes containing this allergen
            intersection.clear();
            let mut is_first = true;
            for ingredient_list in &ingredient_lists {
                if !ingredient_list.allergens.contains(allergen) {
                    continue;
                }
                if is_first {
                    is_first = false;
                    intersection.extend(&ingredient_list.ingredients);
                    for (_, ingredient) in &known_allergens {
                        intersection.remove(*ingredient);
                    }
                } else {
                    intersection.drain_filter(|ingredient| {
                        !ingredient_list.ingredients.contains(ingredient)
                    });
                }
            }
            // If there is only one, we know that this allergen is contained in
            // this ingredient.
            if intersection.len() == 1 {
                known_allergens.push((allergen, *intersection.iter().next().unwrap()));
                pending_allergens.remove(pending_index);
                continue 'outer;
            }
        }
        return Err(Error::NoSolution);
    }

    Ok((ingredient_lists, known_allergens))
}

#[cfg(test)]
const EXAMPLE: &str = "\
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

standard_tests!(
    parse_and_process []
    pt1 [ EXAMPLE => 5 ]
    pt2 [ EXAMPLE => "mxmxvkd,sqjhc,fvjkl" ]
);
