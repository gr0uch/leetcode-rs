use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};
use std::collections::HashMap;
use std::collections::BTreeSet;

#[derive(Default, Debug)]
struct FoodRatings {
    cuisines: HashMap<String, BTreeSet<Food>>,
    food_cuisine: HashMap<String, String>,
    name_food: HashMap<String, Food>,
}

#[derive(Clone, Eq, Debug)]
struct Food {
    name: String,
    rating: i32,
}

impl Ord for Food {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.rating, &other.name).cmp(&(other.rating, &self.name))
    }
}

impl PartialOrd for Food {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.rating != other.rating {
            Some(self.rating.cmp(&other.rating))
        } else {
            Some(other.name.cmp(&self.name))
        }
    }
}

impl PartialEq for Food {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.rating == other.rating
    }
}

impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut r: Self = Default::default();
        for i in 0..foods.len() {
            let name = &foods[i];
            let cuisine = &cuisines[i];
            let rating = ratings[i];
            let food = Food {
                name: name.to_string(),
                rating,
            };
            r.food_cuisine.insert(name.to_string(), cuisine.to_string());
            r.name_food.insert(name.to_string(), food.clone());
            let foods = r.cuisines.entry(cuisine.to_string()).or_insert(BTreeSet::new());
            foods.insert(food);
        }
        r
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        let name = &food;
        let cuisine = self.food_cuisine.get(name).unwrap();
        let foods = self.cuisines.get_mut(cuisine).unwrap();
        let food = self.name_food.get_mut(name).unwrap();
        foods.remove(food);
        food.rating = new_rating;
        foods.insert(food.clone());
    }

    fn highest_rated(&self, cuisine: String) -> String {
        let foods = self.cuisines.get(&cuisine).unwrap();
        foods.iter().last().unwrap().clone().name
    }
}

fn main() {
    println!("ans {:?}", FoodRatings::new(
        ["kimchi", "miso", "sushi", "moussaka", "ramen", "bulgogi"].iter().map(|s| s.to_string()).collect(),
        ["korean", "japanese", "japanese", "greek", "japanese", "korean"].iter().map(|s| s.to_string()).collect(),
        vec![9, 12, 8, 15, 14, 7]
    ));
}
