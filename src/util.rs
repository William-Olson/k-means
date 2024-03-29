//! A module for trivial tasks.
//!
//! # Path
//!
//! util.rs
//!
//! # Description
//!
//! Provides methods for parsing the
//! console arguments (including reading
//! file content) and writing results to
//! a file.

//std libs
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const PROGRAM: &'static str = "k-means";

/// Outputs `msg` to the console.
pub fn output(msg: &str) {
    println!("{}", msg);
}

/// Handles file input and argument parsing.
///
/// # Panics
///
/// If reading the file produces any errors
/// a panic will be called from the `read_file()`
/// function which will terminate the thread. Also
/// if the k parameter can not be parsed into a usize
/// variable then a panic will automatically be called.
pub fn parse_args() -> (String, usize, bool) {
    let mut args = Vec::new();
    let mut tmp_str = String::new();

    // arg length check
    if env::args().len() <= 2 {
        println!("\nError: Not enough arguments!");
        println!("Cargo Usage: \n\n\tcargo run <file> <k>\n");
        println!("Binary Usage: \n\n\t./k-means <file> <k>\n");
        return (tmp_str, 0, true);
    }

    // get args
    for a in env::args() {
        if a.contains(PROGRAM) == false {
            args.push(a);
        }
    }

    // parse k arg
    let k: usize = match args[1].parse::<usize>() {
        Err(e) => {
            eprintln!("\nError parsing k argument: {}", e);
            0
        }
        Ok(result) => result,
    };

    // read file data
    let mut file = read_file(&args[0]);
    match file.read_to_string(&mut tmp_str) {
        Err(er) => eprintln!("\nError reading file: {}", er),
        Ok(_) => { /* data read successfully */ }
    }

    // return errors if needed
    if k == 0 || tmp_str.is_empty() {
        eprintln!("Error: Parsing arguments Failed.");
        println!("Please ensure k > 0, & the file is not empty.");
        return (tmp_str, k, true);
    }

    (tmp_str, k, false)
}

/// Opens a file at the given path `filename`
/// and returns the `File` object.
///
/// # Panics
///
/// If the _open file_ operation ecounters
/// any errors, a panic will be called, which
/// terminates the thread & displays errors.
fn read_file(filename: &String) -> File {
    let path = Path::new(filename);
    let dsp = path.display();
    // open file (in read-only mode) & read contents
    let file: File = match File::open(&path) {
        Err(er) => panic!("Error: couldn't open {}: {}", dsp, er),
        Ok(file) => file,
    };
    file
}

/// Creates a file in the output folder & writes
/// the given String `res` to it. Clobbers if exists.
///
/// # Panics
///
/// If errors occur trying to make the file with the path
/// from `build_filename()`, a panic! will be called from
/// the `mk_file()` function. Also if errors occur trying
/// to write the res string to the file, a panic! will be
/// called from the `wr_str_to_file()` function.
pub fn results_to_file(res: &String) {
    let output_name = build_filename();
    let path = Path::new(&output_name);
    let mut file = mk_file(&path);

    wr_str_to_file(&mut file, res);
}

/// Creates a filename String based on the input filename.
fn build_filename() -> String {
    let mut fname = String::from("output/results");
    for a in env::args() {
        if a.contains(".txt") {
            let parts: Vec<String> = a.split("/").map(|p| p.to_string()).collect();
            fname.push('_');
            fname.push_str(&((parts[parts.len() - 1]).to_string()));
        }
    }
    if fname.len() == 14 {
        fname.push_str(&".txt")
    }
    fname
}

/// Creates a file for writing to with the given path.
///
/// # Panics
///
/// If errors occur trying to create the file with the given
/// path, a panic! will be called, terminating the thread.
fn mk_file(p: &Path) -> File {
    let dsp = p.display();
    let file = match File::create(&p) {
        Err(er) => panic!("Error creating {}: {}", dsp, er),
        Ok(file) => file,
    };
    file
}

/// Writes the given String `instr` to the given File `f`.
///
/// # Panics
///
/// If errors occur trying to write the given String to the
/// specified File, a panic! will be called and the thread
/// will terminate with an error message.
fn wr_str_to_file(f: &mut File, instr: &String) {
    match f.write(instr.as_bytes()) {
        Err(reason) => panic!("Error writing to file: {}", reason),
        Ok(_) => { /* write successful */ }
    }
}
