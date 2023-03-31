use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;
use colored::*;
use rev_lines::RevLines;
use std::env;
// use std::io;
use std::io::{stdin, Write};
// use dirs::*;

use crate::tagesauswahl;

pub fn main(tage: i32) -> Vec<i32> {
    
    //read the commandline arument
    let args: Vec<String> = env::args().collect();

    //take the 2nd argument and pass it into the query variable
    let query = &args[1];

    // Open the file and create a buffered BufReader
    let file = File::open(query).unwrap();

    //maka a usize number from the variable that was passed through
    let cap = usize::try_from(tage).unwrap();

    //use said variable to limit the size of the vector that stores the final values
    let mut numbers = Vec::with_capacity(cap);

    //make use of the rev_lines crate, so that the values shown are always the ones at the bottom 
    //(the latest)
    let rev_lines = RevLines::new(BufReader::new(&file)).unwrap();

    //add the individual values to the vector
    for line in rev_lines.take(cap) {
        let number: i32 = line.parse().unwrap();
        numbers.push(number);
    }

    //return the vector
    numbers
}

//function to add values to the file
pub fn add() {
    //read the commandline arument
    let args: Vec<String> = env::args().collect();

    //take the 2nd argument and pass it into the query variable
    let query = &args[1];

    //promt
    println!("");
    println!("How was your day out of 10?");

    //read user input
    let mut user_input = String::new();
    stdin().read_line(&mut user_input).expect("Failed to read user input!");

    //counter for the numeric check
    let mut count = 0;

    //trim the return sign of the user input
    let trimmed = user_input.trim();

    //check if all characters are numeric
    for c in trimmed.chars() {

        //if not numeric send error and let rety
        if !c.is_numeric() {
            println!("{}", "Input not numeric! Try again".red());

            //give the option to retry
            add();
        } else {

            //update count
            count = count + 1;

            //if gone thrigh all characters
            if count == trimmed.len() {

                //write input to file
                    let mut file = OpenOptions::new().append(true).open(query).expect("Unable to open file"); 
                    file.write_all(user_input.as_bytes()).expect("Failed to write user input!");
                    
                    //show the updated graph
                    tagesauswahl();
            }
        }
    }
}
