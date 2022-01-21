pub struct Meal {
    name: String,
    prep_time: String,
    cost: f32,
    calories: i32,
    eaten: bool
}

impl Meal {
    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    pub fn get_prep_time(&self) -> String {
        return self.prep_time.clone();
    }

    pub fn get_cost(&self) -> f32 {
        return self.cost;
    }

    pub fn get_calories(&self) -> i32 {
        return self.calories;
    }
    
    pub fn is_eaten(&self) -> bool {
        return self.eaten.clone();
    }

    pub fn mark_eaten(&mut self) {
        self.eaten = true;
    }

    pub fn clone(&self) -> Meal {
        return Meal {
            name: self.name.clone(),
            prep_time: self.prep_time.clone(),
            cost: self.cost,
            calories: self.calories,
            eaten: self.eaten
        }
    }
}

pub fn create_meal(name: String, prep_time: String, cost: f32, calories: i32) -> Meal{
    Meal {
        name,
        prep_time,
        cost,
        calories,
        eaten: false
    }
}