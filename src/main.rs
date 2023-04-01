// The GPLv3 License (GPLv3)

// Copyright (c) 2023 Lovis Rentsch

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

//import the other file
pub mod read;

use std::io;
use colored::*;
use termsize;
use std::env;

fn main() {
    //prompt
    println!("enter show(1), add(2) or help(3)");
    
        //read the user-input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        //comupte the input and call the functions
        match input.trim() {
            //always give the option to also use the number
            "help" | "3" => {
                help();
            }

            "show" | "1" => {
                //check if the parameter is passed
                if chack_param() {
                    //if yes, proceed
                    tagesauswahl();

                //else error
                } else {
                    println!("{}", "The file path must be provided as a parameter!".red());
                }
            }
            "add" | "2" => {
                //same as befor
                if chack_param() {
                    read::add();
                } else {
                    println!("{}", "The file path must be provided as a parameter!".red());
                }
            }
            _ => {
                //not here, the help function should always be accessible and it doesn't need the
                //argument
                help();
            }
        }
    // }
}

//first step of the show option
pub fn tagesauswahl() {
    //prompt
    println!("");
    println!("How many days should be displayed (0 for the max length)?");   

    //read awnser
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    //format the awnser (remove the return at the end)
    let trimmed_input = input.trim();

    //check if all the given options are numeric
    let mut count = 0;
    for c in trimmed_input.chars() {
        //of one of them isn't give an error and let the user try again
        if !c.is_numeric() {
            println!("{}", "Input can only be numeric. Try again!".red());
            tagesauswahl();

            //if no error is thrown (no char that isn't a nuber)
        } else {

            let mut breite = 1;
            termsize::get().map(|size| {
                breite = size.cols;
            });

            // (this couner is needed to make sure that all the chars (letters/numbers) are numbers
            // (and not just the first one))
            count = count + 1;
            
            //if ALL the chars are numbers -> checking the counter
            if  count == trimmed_input.len() {

                //format "num" to be an i32 number (for further use)
                let num: i32 = trimmed_input.parse().unwrap();

                //check if the requested number is longer than the possible amount
                if read::main(num).len() < usize::try_from(num).unwrap() {

                    //if that is the case error and let the user try again
                    println!("{}", "Daycount cannot be bigger than total recorded days! Try a smaller value.".red());
                    tagesauswahl();

                    //else go on with the selection of the topic while passing in the number of
                    //days (for further use)
                } else if ((num + 8) * 2) > (breite as i32) {
                    println!("{}", "Value cannot be bigger than the terminal width! Try a smaller value.".red());
                    tagesauswahl();
                } else {
                    //A row to seperate the user input from the graph
                    println!("");
                    
                    //if input is 0 the max possible length should be displayed
                    if num == 0 {
                        //use the smaller value
                        //between the line number of the file and the width of the terminal
                        //-> no overflow
                        if read::linecount() > breite as i32 / 2 {
                            graph(breite as i32 / 3);
                        } else {
                            graph(read::linecount());
                        }
                    } else {
                        //if it is any other number do it the normal way
                        graph(num);
                    }

                    //call the graph with the number of values which should be parsed
                    // graph(num);
                }
            }
        }
    }
}

//function that draws the output to the terminal
fn graph(tage: i32) {
    
    //variable thaz counts the times that the loop ran (the row we are in)
    let mut lauf = 1;
    
    //giving the output a height of 10 
    //starting at the bottom (rev) so that the highest number has the highest pillar (not upside down)
    open_unten_rahmen(tage);

    //call the read function to init the vec variable
    //PERFORMANCE!
    let vec = read::main(tage);

    for zeile in (1..=10).rev() {
        print!("{}", " |  ".blue());
        
        //numbering the x-Axis
        //making sure that all numbers are 2 chars wide (else issues with the 10)
        let width: usize = 2;
        print!("  {zeile:>width$}  ");
       

        //adding 1 to the global row count
        lauf = lauf + 1;

        //looping through all the numbers in the vector
        //the vector comes from the "read.rs" file (and it's main function)
        for stelle in (0..vec.len()).rev() {

            //if the value of the current number is bigger than the number of the row
            //then print a red rectangle
            if vec[stelle] >= zeile {

                //burrow the value to the color function
                farbe(&lauf);
            } else {

                //if the value is 0 paint the whole collumn in black
                if vec[stelle] == 0 {
                    print!("{}", "  ".on_black());

                    //this is so that the top (not used part) is formatted correctly
                } else {
                    print!("  ");
                }
            }
        }

        print!("{}", " |   ".blue());
        //line breaks at the end of each line
        print!("\n");
    }
    open_unten_rahmen(tage); 
}

fn farbe(lauf: &i32) {

    //check the values and assign diffetent values to achive a vertical gradient
    match lauf {
        1 => print!("{}", "  ".on_truecolor(39, 163, 10)),
        2 => print!("{}", "  ".on_truecolor(62, 154, 9)),
        3 => print!("{}", "  ".on_truecolor(85, 145, 8)),
        4 => print!("{}", "  ".on_truecolor(107, 135, 7)),
        5 => print!("{}", "  ".on_truecolor(130, 126, 6)),
        6 => print!("{}", "  ".on_truecolor(153, 117, 4)),
        7 => print!("{}", "  ".on_truecolor(176, 108, 3)),
        8 => print!("{}", "  ".on_truecolor(198, 98, 2)),
        9 => print!("{}", "  ".on_truecolor(221, 89, 1)),
        _ => print!("{}", "  ".on_truecolor(244, 80, 0)),
    };
}

//telling the user how to use the programm
fn help() {
    println!("");
    println!("{}" ,"This script can be used to track all sorts of progress.".bold());
    println!("");
    println!("It is required to pass the path to the file that should be read as a commandline argument.");
    println!("  {}" ,"./tracker /home/example/log.txt".on_black().yellow());
    println!("Like that it is possible to track multiple different things by distributing them into different files.");
    println!("");
    println!("Form the initial prompt, you can choose to show the log of the progress you tracked in a graph (choose the 1), or to add an entry (choose the 2).");
    println!("");
    println!("Colors: (from good to bad)");
    for n in (1..=10).rev() {
        print!("{n:2} ");
    }
    print!("\n");
    for n in 1..=10 {
        farbe(&n);
        print!("|")
    }
    println!("");
    println!("");
    println!("If a {} is given as an argument the day is marked as «not counted» and displayed as {}.", "0".bold() , "  ".on_black());
    println!("");
    println!("Version 1.2.2");
}


fn open_unten_rahmen(tage: i32) {
    //add the left margin
    print!("{}"," ----".blue());
    for _n in 0..=tage {
        
        //draw as many as days
        print!("{}", "--".blue());
    }


    //add right margin
    print!("{}", "----".blue());

    //end line
    print!("\n");
}

//function to check if the file path is passed as an argument
fn chack_param() -> bool {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        false
    } else {
        true
    }
}
