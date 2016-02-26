/*
* Program:     k-means (hw2)
* Author:      Will Olson
* Date:        2/25/2016
*
* File: util.rs
*/

//some std libs
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

//parse the args
pub fn parse_args () -> (String, f32, bool) {
  let mut args = Vec::new();
  let tmp = String::new();

  //arg len check
  if env::args().len() < 2 {
    return (tmp, -1.0, true);
  }

  //get args
  for a in env::args() {
    if a.contains(PROGRAM) == false {
      args.push(a);
    }
  }

  //parse k arg
  let k: f32 = match args[1].parse::<f32>() {
      Err(e) => { println!("Error parsing k argument: {}", e.to_string()); -1.0 },
      Ok(result) => result,
  };

  (String::from("test"), k, false)
}
