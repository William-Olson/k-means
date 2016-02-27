/*
 * Program:     k-means (hw2)
 * Author:      Will Olson
 * Date:        2/25/2016
 *
 * File: main.rs
*/

// compiler verbosity config
#![warn(missing_docs)]
#![allow(dead_code)]


/// for arg parsing and file io
mod util;

/// For interacting with data/cluster sets
mod worker;

// shorten namespaces
use util::*;
use worker::Worker;

/// Runs the k-means program.
fn main() {
    let mut w = Worker::new();
    let (data_blob, k, err) = parse_args();

    if err { return; }
    output("---- k-means ----");
    // show_input(&data_blob, k);

    // set up data structures
    w.set_data(&data_blob);
    w.set_clusters(k);

    // display stucts to console
    w.print_data();
    w.print_clusters();
    w.print_mean_dists(1);
}

/// Prints the user input to the console.
fn show_input (ds: &String, k: usize) {
    println!("k: {} \ndata: \n{}", k, ds);
}
