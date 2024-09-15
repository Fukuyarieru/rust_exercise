#![allow(dead_code)]
use text_io::*;

fn main() {
    loop {
        println!("Enter the first number, operation, and second number:");

        let first: String;
        let operation: String;
        let second: String;

        // Scan the input for first number, operation, and second number
        scan!("{} {} {}", first, operation, second);

        // Create a new instance of NormalExercise using the input
        let ex: NormalExercise = NormalExercise::new(first, operation, second);

        // Display the result
        println!("Complete: {}", ex.complete); // e.g., "5 + 3 = 8"
        println!("Answer: {}", ex.answer); // e.g., "8"

        println!("Would you like to continue?: (y/n)");
        let mut input: String = read!("{}");
        while input != "y" && input != "n" {
            println!("Wrong input. Try again: (y/n)");
            input = read!("{}");
        }
        match input.as_str() {
            "y" => (),
            "n" => break,
            _ => (),
        }
    }
}

struct NormalExercise {
    first: String,
    operation: String,
    second: String,
    answer: String,
    complete: String,
}

struct SpecialExercise {
    number_list: Vec<String>,
    operations_list: Vec<String>,
    answer: String,
    complete: String,
}

impl NormalExercise {
    // Constructor method for NormalExercise
    fn new(first: String, operation: String, second: String) -> NormalExercise {
        // Parse the first and second strings as floating-point numbers
        let first_num: f64 = first.parse().expect("Invalid number in 'first'");
        let second_num: f64 = second.parse().expect("Invalid number in 'second'");

        // Determine the result based on the operation
        let result: String = match operation.as_str() {
            "+" => (first_num + second_num).to_string(),
            "-" => (first_num - second_num).to_string(),
            "*" => (first_num * second_num).to_string(),
            "/" => {
                if second_num != 0.0 {
                    (first_num / second_num).to_string()
                } else {
                    String::from("ERROR")
                }
            }
            _ => panic!("Unsupported operation"),
        };

        // Create the complete equation string
        let complete = format!("{} {} {} = {}", first, operation, second, result);

        // Return a new NormalExercise instance
        NormalExercise {
            first,
            operation,
            second,
            answer: result.to_string(),
            complete,
        }
    }
}

impl SpecialExercise {}
