use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::ErrorKind;
use colored::*;
use rev_lines::RevLines;
use std::env;
use std::io::{stdin, Write, BufRead};
// use std::error::ErrorKind;

use crate::tagesauswahl;
use crate::homedir;

pub fn main(tage: i32) -> Vec<i32> {
    
    //read the commandline arument
    let args: Vec<String> = env::args().collect();
    let query = if args.len() < 2 {
        let q = format!("{}{}", homedir(), "/.tracker");
        q
    } else {
        let q = match env::args().nth(0) {
            Some(v) => v,
            None => panic!("Error preparing file opening!"),
        };
        q
    };
    //take the 2nd argument and pass it into the query variable

    // Open the file and create a buffered BufReader
    let file = File::open(&query);

    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(&query) {
                    Ok(fc) => {
                    println!("{}", "Due to missing file a file was created.".green());
                    fc
                },
                    Err(error) => panic!("Error reading and creating the file: {}", error),
            },
            ErrorKind::Other => {
                panic!("Unexpected error while reading file");
            },
            _ => panic!("Unexpected error while reading file"),
        },
    };

    //maka a usize number from the variable that was passed through
    let cap = usize::try_from(tage).unwrap();

    //use said variable to limit the size of the vector that stores the final values
    let mut numbers = Vec::with_capacity(cap);

    //make use of the rev_lines crate, so that the values shown are always the ones at the bottom 
    //(the latest)
    let rev_lines = RevLines::new(BufReader::new(&file)).unwrap();

    //add the individual values to the vector
    for line in rev_lines.take(cap) {
        // let number: i32 = line.parse().unwrap();
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
    let query = if args.len() < 2 {
        let q = format!("{}{}", homedir(), "/.tracker");
        q
    } else {
        let q = match env::args().nth(0) {
            Some(v) => v,
            None => panic!("Error preparing file opening!"),
        };
        q
    };

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
                    let mut file = OpenOptions::new().append(true).open(&query).expect("Unable to open file"); 
                    file.write_all(user_input.as_bytes()).expect("Failed to write user input!");
                    
                    //show the updated graph
                    tagesauswahl();
            }
        }
    }
}

pub fn linecount() -> i32 {
    // read the commandline argument (the file path)
    let args: Vec<String> = env::args().collect();
    let query = if args.len() < 2 {
        let q = format!("{}{}", homedir(), "/.tracker");
        q
    } else {
        let q = match env::args().nth(0) {
            Some(v) => v,
            None => panic!("Error preparing file opening!"),
        };
        q
    };
    let file = BufReader::new(File::open(query).expect("Unable to open file"));

    let mut cnt = 0;

    for _ in file.lines() {
        cnt = cnt + 1;
    }

    //return the line number
    cnt
}

pub fn create_file(path: String) {
    let file = File::open(&path);

    match file {
        Ok(file) => {
            println!("{}", "File already exists. Please run the programm with the path as an argument.".yellow());
            file
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(&path) {
                    Ok(fc) => {
                    println!("{}'{}'{}", "Wrote File ".green(), path, ". Please rerun the programm.".green());
                    fc
                },
                    Err(error) => panic!("Error reading and creating the file: {}", error),
            },
            ErrorKind::Other => {
                panic!("Unexpected error while reading file");
            },
            _ => panic!("Unexpected error while reading file"),
        },
    };
}
