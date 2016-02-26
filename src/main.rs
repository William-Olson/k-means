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

    //show read file data & k value
    let mut arg_info = String::new();
    arg_info.push_str("k: ");
    arg_info.push_str(&(k.to_string()));
    arg_info.push_str("\ndata\n");
    arg_info.push_str(&data_blob);
    output(&arg_info);


    w.set_clusters(k);

}
