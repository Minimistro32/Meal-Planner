# Overview
This project is a meal planner CLI built entirely with Rust, to learn Rust. Everyone gets hungry. The question always ends up being, "What's for dinner?" With this CLI you can schedule meals and keep an organized recipe box.

This project enabled me to learn and utilize Rust features. Custom meal data made Structures a logical choice. Ownership came naturally as I began passing information between functions and into data structures. 

My existing technological repertoire ranges from Python to Swift to MS-SQL. I've already had some exposure to C++ but could feel I was lacking in the high performance code department. That is why this project exists, as an introduction to an enjoyable high performance language.

[Software Demo Video](https://youtu.be/dq3RcKxoogA)

# Development Environment
## Tools
* VS Code (Rust Extension)
* Rustc
* Cargo

VS Code is a popular IDE. The [Rust Extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust) makes development convenient. 

[Rustc](https://doc.rust-lang.org/rustc/index.html) is the rust compiler. It is usually invoked using the next tool.

Cargo is the rust package manager and build tool all rolled into one. The command ```cargo check``` allows you to check for errors without building the project. To build and run the project use ```cargo run```.

## Language
Rust was voted the [most loved](https://insights.stackoverflow.com/survey/2021#section-most-loved-dreaded-and-wanted-programming-scripting-and-markup-languages) programming language for the sixth year in a row in Stack Overflow's annual developer survey. It continues to build on the legacy of C like many other languages; however, this time with an emphasis on compile-time safety and speed.

Rust's most innovative feature is ownership. Ownership is a compile-time feature designed to prevent *potential* run-time errors. The enforced rules are simple:
* Each value in Rust has a variable that’s called its owner.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.

# Useful Websites

The first link, nicknamed "the book", was invaluable for this short project. My first step in development was to read the first eight chapters. That was a sufficient primer to jump right in. As I started to cement my reading with experience, I continued to supplement gaps in understanding with additional chapters and Stack Overflow questions. These two resources exclusively gave me the knowledge necessary to produce this CLI.
* [The Rust Programming Language](https://doc.rust-lang.org/book/)
* [Stack Overflow - Rust](https://stackoverflow.com/questions/tagged/rust)

# Future Work

Continuing this project beyond the initial scope of two weeks would likely include adding the following features:
* Changing to a grid based calendar system.
* Refactoring the input function to accept a text prompt parameter.
* Including the ability to add recipes to meals.
* Persisting calendar and recipe_box between executions.