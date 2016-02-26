/*
* Program:     k-means (hw2)
* Author:      Will Olson
* Date:        2/25/2016
*
* File: main.rs
*
*/


#![allow(dead_code)]

// local modules
mod util;
mod worker;
use util::*;
use worker::Worker;

fn main() {
    let mut w = Worker::new();
    let (data_blob, k, err) = parse_args();
    if err { return; }

    output("---- k-means ----");

    // show_input(&data_blob, k);

    w.set_data(&data_blob);
    w.set_clusters(k);
    
    w.print_data();
    w.print_clusters();

}


fn show_input (ds: &String, k: usize) {
    println!("k: {} \ndata: \n{}", k, ds);
}
