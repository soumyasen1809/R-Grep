/*
https://exercism.org/tracks/rust/exercises/grep
How to compile: search in poem.txt the word "arTicle"
cargo run -r -- src/poem.txt arTicle
*/

use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

enum GrepFlags {
    N, // -n Print the line numbers of each matching line.
    I, // -i Match line using a case-insensitive comparison.
    V, // -v Invert the program -- collect all lines that fail to match the pattern.
    X, // -x Only match entire lines, instead of lines that contain a match.
}

impl GrepFlags {
    fn choose_function(&self, filecontents: BufReader<File>, search_phrase: &String) -> () {
        match self {
            Self::N => pattern_match(filecontents, &search_phrase, "N".to_string()),
            Self::I => pattern_match(filecontents, &search_phrase, "I".to_string()),
            Self::V => pattern_match(filecontents, &search_phrase, "V".to_string()),
            Self::X => pattern_match(filecontents, &search_phrase, "X".to_string()),
        }
    }
}

fn pattern_match(filecontents: BufReader<File>, search_phrase: &String, flag_used: String) {
    /*
    you can't do pattern matching on a String.
    You could expect a success, get a &str from the String then pattern match on that
    */
    match flag_used.as_ref() {
        "I" => {
            for line in filecontents.lines() {
                let line: String = line.unwrap();
                /*
                https://doc.rust-lang.org/std/?search=to_lowercase
                to_lowercase converts entire string to lowercase characters
                */
                let index: Option<usize> =
                    str::to_lowercase(&line).find(str::to_lowercase(&search_phrase).trim());

                match index {
                    Some(t) => {
                        println!("Found the word '{search_phrase}' at position {t} of line: {line}")
                    }
                    None => (),
                }
            }
        }

        "V" => {
            for line in filecontents.lines() {
                let line: String = line.unwrap();
                /*
                https://doc.rust-lang.org/std/?search=to_lowercase
                to_lowercase converts entire string to lowercase characters
                */
                let index: Option<usize> =
                    str::to_lowercase(&line).find(str::to_lowercase(&search_phrase).trim());

                match index {
                    Some(_t) => (),
                    None => {
                        println!("Did not find the word '{search_phrase}' at line: {line}")
                    }
                }
            }
        }

        "N" => {
            let mut linenumber: isize = 0; // initilize counting line number to 0
            for line in filecontents.lines() {
                let line: String = line.unwrap();
                let index = line.find(&search_phrase.trim());
                linenumber += 1; // increment line number every time a line is read

                match index {
                    Some(_t) => {
                        println!("Found the word {search_phrase} at line number {linenumber}");
                    }
                    None => (),
                }
            }
        }

        "X" => {
            for line in filecontents.lines() {
                let line: String = line.unwrap();
                /*
                https://doc.rust-lang.org/std/?search=to_lowercase
                to_lowercase converts entire string to lowercase characters
                */
                if line.trim() == search_phrase.trim() {println!("The entire line is the word: {line}");}
                else {}
            }
        }

        _ => println!("Wrong choice"),
    }
}

fn read_file_line(filepath: &String) -> BufReader<File> {
    let data: File = File::open(&filepath).unwrap();
    let buffer: BufReader<File> = BufReader::new(data);

    buffer
}

fn main() {
    // Accepting Command Line Arguments
    let args: Vec<String> = env::args().collect();
    let filepath: &String = &args[1];
    let search_arg: &String = &args[2];
    let search_phrase: String = search_arg.to_string();

    let filecontents: BufReader<File> = read_file_line(&filepath);
    GrepFlags::N.choose_function(filecontents, &search_phrase);

    let filecontents: BufReader<File> = read_file_line(&filepath); // need to repeat, else filecontents is moved and can't be reused
    GrepFlags::I.choose_function(filecontents, &search_phrase);

    let filecontents: BufReader<File> = read_file_line(&filepath);
    GrepFlags::V.choose_function(filecontents, &search_phrase);

    let filecontents: BufReader<File> = read_file_line(&filepath);
    GrepFlags::X.choose_function(filecontents, &search_phrase);
}
