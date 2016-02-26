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
    let n: usize = 10;

    let (data_blob, k, err) = parse_args();
    if err { return; }

    println!("k-means: {:?}", k);
    output(&data_blob);
    w.set_clusters(n);

}
