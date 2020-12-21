use std::{collections::{HashMap, HashSet}, str::FromStr};

pub mod part1 {
  use super::*;
  use anyhow::*;
  pub fn solve(input: Input) -> Result<usize> {
    let safe_ingredients = input.safe_ingredients();
    let count = input.foods
      .iter()
      .map(|f| {
        f.ingredients
          .iter()
          .filter(|&i| safe_ingredients.contains(i))
          .count()
      }).sum();
    return Ok(count);
  }
}

pub struct Food {
  ingredients: HashSet<String>,
  allergens: HashSet<String>,
}
impl FromStr for Food {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut ingredients = HashSet::new();
    let mut allergens = HashSet::new();

    let mut split = s.split('(');
    let (ingr, rest) = (split.next().unwrap(), split.next().unwrap());
    
    ingredients.extend(
      ingr.split(' ')
               .map(|s| s.trim().to_string())
               .filter(|s| !s.is_empty())
    );
    allergens.extend(
      rest["contains".len()..]
            .split(',')
            .map(|s| s.trim())
            .map(|s| s.to_string())
            .map(|s| if s.ends_with(")") { s[..s.len() - 1].to_string() } else { s })
            .filter(|s| !s.is_empty())
    );
    Ok(Food { ingredients, allergens })
  }
}

pub struct Input {
  foods: Vec<Food>,
}
impl FromStr for Input {
  type Err = ();
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Input { foods: s.lines().map(|s| s.parse::<Food>().unwrap()).collect() })
  }
}

impl Input {
  pub fn safe_ingredients(&self) -> HashSet<String> {
    let mut possible_ingredients = HashMap::<String, HashSet::<String>>::new();
    let ingredients = self.foods.iter()
      .map(|f| &f.ingredients)
      .fold(HashSet::new(),
            |mut ing, i| {
              ing.extend(i.clone());
              ing
            });
    for food in &self.foods {
      for allergen in &food.allergens {
        let hs = possible_ingredients.remove(allergen).unwrap_or_else(|| ingredients.clone());
        possible_ingredients.insert(allergen.clone(), hs.intersection(&food.ingredients).cloned().collect());
      }
    }

    let mut safe_ingredients = ingredients.clone();
    for (_allergen, ingredients) in possible_ingredients {
      safe_ingredients = safe_ingredients.difference(&ingredients).cloned().collect();
    }

    return safe_ingredients;
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  pub fn example() {
    let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
    trh fvjkl sbzzf mxmxvkd (contains dairy)
    sqjhc fvjkl (contains soy)
    sqjhc mxmxvkd sbzzf (contains fish)".parse::<Input>().unwrap();
    input.safe_ingredients();
  }
}