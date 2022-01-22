use std::io;

//MENU FUNCTIONS
pub fn welcome() {
    println!("Welcome to MEAL PLANNER");
}

pub fn main_menu() {
    println!("\n-----------------------");
    println!("------ Main Menu ------");
    println!("-----------------------");
    println!("T. What do I eat today?");
    println!("V. View your Meal Plan");
    println!("P. Plan more meals");
    println!("R. Open your Recipe Box");
    println!("Q. Quit the program");
    prompt_selection("\nPlease select an option: ");
}

pub fn recipe_box_menu() {
    println!("\n-----------------------");
    println!("------ Recipe Box -----");
    println!("-----------------------");
    println!("V. View a Genre of Meals");
    println!("A. Add a Genre");
    println!("R. Remove a Genre");
    println!("Q. Quit the Recipe Box");
    prompt_selection("\nPlease select an option: ");
}

pub fn genre_menu() {
    println!("\n-----------------------");
    println!("-------- Genre --------");
    println!("-----------------------");
    println!("V. View a Meal");
    println!("A. Add a Meal");
    println!("R. Remove a Meal");
    println!("Q. Quit this Genre");
    prompt_selection("\nPlease select an option: ");
}

//USER INTERFACE FUNCTIONS
pub fn prompt_selection(prompt: &str) {
    print!("{}", prompt);
    io::Write::flush(&mut io::stdout()).expect("flush failed!");
}

pub fn input() -> String {
    let mut selection = String::new();
    io::stdin()
        .read_line(&mut selection)
        .expect("Failed to read line");
    return selection;
}