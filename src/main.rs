mod menu;
mod model;

use std::collections::hash_map;
use std::collections::HashMap;
use std::io;

use model::Meal;

//MAIN FUNCTIONS
fn main() {
    menu::welcome();

    //Initialize a sample recipe_box (a production version would save this to disk)
    let mut recipe_box: HashMap<String, Vec<Meal>> = HashMap::new();
    let meals = recipe_box
        .entry(String::from("Sandwiches"))
        .or_insert(Vec::new());

    meals.push(model::create_meal(
        String::from("Meatball Sub"),
        String::from("20 Minutes"),
        15.00,
        250,
    ));

    recipe_box
        .entry(String::from("Mexican"))
        .or_insert(Vec::new());

    //Initialize a sample calendar (a production version would save this to disk)
    let mut calendar: Vec<Vec<Option<Meal>>> = Vec::new();
    add_week(&mut calendar);

    loop {
        menu::main_menu(); //display the main_menu

        match menu::input().to_uppercase().trim() {
            //get menu option
            "T" => {
                if let Some(today) = &mut calendar[0][0] {
                    println!("Today's meal is: {}", today.get_name()); //display meal for today

                    menu::prompt_selection(
                        "\nWould you like to mark off today on the calendar? (Y/N)\n",
                    ); //ask if they want to mark today as complete
                    loop {
                        match menu::input().to_uppercase().trim() {
                            "Y" => {
                                today.mark_eaten();
                                break;
                            }
                            "N" => {
                                break;
                            }
                            _ => {
                                continue;
                            }
                        }
                    }
                } else {
                    println!("There is no meal planned for today.")
                }

                //if the whole week is eaten delete it
                let first_week = &calendar[0];
                let mut is_complete = true;
                for day in first_week {
                    if let Some(meal) = day {
                        if !meal.is_eaten() {
                            is_complete = false;
                        }
                    } else {
                        is_complete = false;
                    }
                }

                if is_complete {
                    calendar.remove(0);
                }
            }
            "V" => {
                display_calendar(&calendar); //view the calendar
            }
            "P" => {
                if recipe_box.keys().len() > 0 {
                    let mut is_empty = true;
                    if let Some(last_week) = calendar.last() {
                        //get last vector in calendar
                        for meal in last_week {
                            if !meal.is_none() {
                                is_empty = false;
                            }
                        }
                    }
                    if !is_empty {
                        //if it has even one element in it add another vector with 7 None: Option<Meal>.
                        add_week(&mut calendar);
                    }

                    display_calendar(&calendar);
                    print!("\n");

                    let week_index: usize;
                    loop {
                        menu::prompt_selection("Select a week: ");
                        match menu::input().trim().parse::<usize>() {
                            //verify the selected index is valid
                            Ok(index) => {
                                if index < 1 || index > calendar.len() {
                                    continue;
                                }
                                week_index = index - 1;
                                break;
                            }
                            Err(_) => {
                                continue;
                            }
                        }
                    }

                    let day_index: usize;
                    loop {
                        menu::prompt_selection("Select a day: ");
                        match menu::input().trim().parse::<usize>() {
                            Ok(index) => {
                                if index < 1 || index > calendar[week_index].len() {
                                    continue;
                                }
                                day_index = index - 1;
                                break;
                            }
                            Err(_) => {
                                continue;
                            }
                        }
                    }

                    menu::prompt_selection("\nSelect a genre of meal: ");
                    let genre = pick_genre(recipe_box.keys());

                    if let Some(meals) = recipe_box.get(&genre) {
                        if meals.len() > 0 {
                            menu::prompt_selection("\nSelect a meal: ");
                            calendar[week_index][day_index] =
                                Some(meals[pick_meal(&meals)].clone()); //add the selected meal to the selected calendar date
                        } else {
                            println!("\nThis genre doesn't contain any meals yet.");
                        }
                    }
                } else {
                    println!("\nThe Recipe Box doesn't contain any meal genres yet.");
                }
            }
            "R" => {
                recipe_box_main(&mut recipe_box); //go one menu level deeper
            }
            "Q" => {
                break;
            }
            _ => {
                continue;
            }
        }
    }
}

fn recipe_box_main(recipe_box: &mut HashMap<String, Vec<Meal>>) {
    loop {
        menu::recipe_box_menu();

        match menu::input().to_uppercase().trim() {
            "V" => {
                if recipe_box.keys().len() > 0 {
                    if let Some(meals) = recipe_box.get_mut(&pick_genre(recipe_box.keys())) {
                        genre_main(meals);
                    }
                } else {
                    println!("\nThe Recipe Box doesn't contain any meal genres yet.");
                }
            }
            "A" => {
                recipe_box
                    .entry(String::from(menu::input().trim()))
                    .or_insert(Vec::new());
            }
            "R" => {
                recipe_box.remove(&pick_genre(recipe_box.keys()));
            }
            "Q" => {
                break;
            }
            _ => {
                continue;
            }
        }
    }
}

fn genre_main(meals: &mut Vec<Meal>) {
    loop {
        menu::genre_menu();

        match menu::input().to_uppercase().trim() {
            "V" => {
                if meals.len() > 0 {
                    let meal_to_display = &meals[pick_meal(meals)];
                    println!("\nName: {}", meal_to_display.get_name());
                    println!("Prep Time: {}", meal_to_display.get_prep_time());
                    println!("Cost: ${}", meal_to_display.get_cost());
                    println!("Calories: {}", meal_to_display.get_calories());
                } else {
                    println!("\nThis genre doesn't contain any meals yet.");
                }
            }
            "A" => {
                menu::prompt_selection("\nEnter a Name: ");
                let name = String::from(menu::input().trim());

                menu::prompt_selection("Enter a Prep Time: ");
                let prep_time = String::from(menu::input().trim());

                let cost;
                loop {
                    menu::prompt_selection("Enter a Cost: ");
                    match menu::input().trim().parse::<f32>() {
                        Ok(parsed_f32) => {
                            cost = parsed_f32;
                            break;
                        }
                        Err(_) => {
                            continue;
                        }
                    }
                }

                let calories;
                loop {
                    menu::prompt_selection("Enter a Calories: ");
                    match menu::input().trim().parse::<i32>() {
                        Ok(parsed_i32) => {
                            calories = parsed_i32;
                            break;
                        }
                        Err(_) => {
                            continue;
                        }
                    }
                }

                meals.push(model::create_meal(name, prep_time, cost, calories));
            }
            "R" => {
                meals.remove(pick_meal(meals));
            }
            "Q" => {
                break;
            }
            _ => {
                continue;
            }
        }
    }
}

//CALENDAR
fn display_calendar(calendar: &Vec<Vec<Option<Meal>>>) { //by reference because we don't want to own or modify the calendar
    for (i, week) in calendar.iter().enumerate() {
        println!("\nWeek {}", i + 1);
        println!("-------");
        for (day, meal) in week.iter().enumerate() {
            if let Some(meal) = meal {
                if meal.is_eaten() {
                    println!("#. {}", meal.get_name());
                } else {
                    println!("{}. {}", day + 1, meal.get_name());
                }
            } else {
                println!("{}. ", day + 1);
            }
        }
    }
    io::Write::flush(&mut io::stdout()).expect("flush failed!");
}

fn add_week(calendar: &mut Vec<Vec<Option<Meal>>>) { //by reference because we want ownership to stay in main
    calendar.push(Vec::new());
    if let Some(last_week) = calendar.last_mut() {
        for _ in 0..7 {
            last_week.push(None);
        }
    }
}

//OPTION SELECTION
//takes a keys from the recipe_box hash_map
//returns the key of the meal genre the user selects
pub fn pick_genre(mut genres: hash_map::Keys<'_, String, Vec<Meal>>) -> String {
    print!("\n");
    for (i, genre) in genres.clone().enumerate() {
        println!("{}. {}", i + 1, genre);
    }
    loop {
        menu::prompt_selection("\nPlease select an option: ");

        let selection_index: usize;
        match menu::input().trim().parse::<usize>() {
            Ok(index) => {
                if index < 1 || index > genres.len() {
                    continue;
                }
                selection_index = index - 1;
            }
            Err(_) => {
                continue;
            }
        }

        if let Some(selection) = genres.nth(selection_index) {
            return String::from(selection);
        }
    }
}

//takes a vector of meals
//returns the index of the meal the user selects
pub fn pick_meal(meals: &Vec<Meal>) -> usize {
    print!("\n");
    for (i, meal) in meals.iter().enumerate() {
        println!("{}. {}", i + 1, meal.get_name());
    }
    loop {
        menu::prompt_selection("\nPlease select an option: ");
        let selection_index: usize;
        match menu::input().trim().parse::<usize>() {
            Ok(index) => {
                if index < 1 || index > meals.len() {
                    continue;
                }
                selection_index = index - 1;
            }
            Err(_) => {
                continue;
            }
        }

        return selection_index;
    }
}
