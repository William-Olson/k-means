/*
 * Program:     k-means (hw2)
 * Author:      Will Olson
 * Date:        2/25/2016
 *
 * File: main.rs
*/

// compiler verbosity config
#![allow(dead_code)]

// local mods
mod util;
mod worker;
use util::*;
use worker::Worker;

/// Runs the k-means program.
fn main() {
    output("---- k-means ----");

    // parse arguments
    let (data_blob, k, err) = parse_args();
    if err {
        return;
    }

    // create worker from input
    let mut w = Worker::new(data_blob, k);

    w.run(); // run k-means algorithm

    // output results
    let results = w.results_to_string();
    results_to_file(&results);
    output(&results);
}

/// Prints the user input to the console.
fn show_input(ds: &String, k: usize) {
    println!("k: {} \ndata: \n{}", k, ds);
}
