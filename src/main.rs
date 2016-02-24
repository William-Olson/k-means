#![allow(dead_code)]

// local modules
mod util;
mod worker;
use util::*;
use worker::Worker;

fn main() {
    let mut w = Worker::new();
    let k: usize = 10;
    
    outputs("k-means"); 
    w.set_clusters(k);

}
