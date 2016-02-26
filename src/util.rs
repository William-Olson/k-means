/*
* Program:     k-means (hw2)
* Author:      Will Olson
* Date:        2/25/2016
*
* File: util.rs
*/

//std libs
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::env;


const PROGRAM: &'static str  = "k-means";

//output a String ref
pub fn output (msg: &str) {
  println!("{}", msg);
}

//parse_args: handles the input parsing
pub fn parse_args () -> (String, usize, bool) {
  let mut args = Vec::new();
  let mut tmp_str = String::new();

  //arg length check
  if env::args().len() <= 2 {
    println!("\nError: Not enough arguments!");
    println!("Cargo Usage: \n\n\tcargo run <file> <k>\n");
    println!("Binary Usage: \n\n\t./k-means <file> <k>\n");
    return (tmp_str, 0, true);
  }

  //get args
  for a in env::args() {
    if a.contains(PROGRAM) == false {
      args.push(a);
    }
  }

  //parse k arg
  let k: usize = match args[1].parse::<usize>() {
      Err(e) => { println!("\nError parsing k argument: {}", Error::description(&e)); 0 },
      Ok(result) => result,
  };

  //read file data
  let mut file = read_file(&args[0]);
  match file.read_to_string(&mut tmp_str) {
      Err(er) => println!("\nError reading file: {}", Error::description(&er)),
      Ok(_)   => { /* data read successfully */ },
  }

  //return errors if needed
  if k == 0 || tmp_str.is_empty() {
    return (tmp_str, k, true);
  }

  (tmp_str, k, false)
}

fn read_file(filename: &String) -> File {
  let path = Path::new(filename);
  let dsp = path.display();
  // Open the path (in read-only mode) & read contents
  let file: File = match File::open(&path) {
      Err(er) => panic!("Error: couldn't open {}: {}", dsp, Error::description(&er)),
      Ok(file) => file,
  };
  file
}
