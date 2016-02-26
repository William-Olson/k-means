/*
* Program:     k-means (hw2)
* Author:      Will Olson
* Date:        2/25/2016
*
* File: worker/mod.rs
* 
* Methods:
*    calc_means
*    choose_centroids
*    cluster_index
*    data_index
*    new
*    print_clusters
*    print_data
*    run
*    set_clusters
*    set_data
*/


//imports
extern crate rand; // for random number generation
use worker::rand::distributions::{IndependentSample, Range};

//local modules
mod cluster;
mod data_object;
use worker::cluster::*;
use worker::data_object::*;


//this module's struct
pub struct Worker {
  clusters: Vec<Cluster>,
  data_set: Vec<DataObject>,
  convergence: bool
}


//methods for Worker
impl Worker {

  /* new: creates a new worker struct */
  pub fn new () -> Worker {
    let wkr = Worker {
      clusters: Vec::new(),
      data_set: Vec::new(),
      convergence: false
    };
    wkr
  }

  pub fn set_data (&mut self, blob: &String) {
    let ref_data: Vec<&str> = blob.split("\n").collect(); //split by line
    let mut id = 1;
    for line in &ref_data { // split by whitespace
      if line.is_empty() == false {
        // parse vals, create object & add to vector
        self.data_set.push(
            DataObject::new(id, &( line.split_whitespace().map(
                         |val| val.parse::<f32>().unwrap()).collect() )
            )
        );
        id += 1;
      }
    }
  }

  pub fn print_data (&self) {
    for d in &self.data_set { d.print(); }
  }

  /* set_clusters: creates the vector of clusters */
  pub fn set_clusters (&mut self, k: usize) {
    self.clusters = Vec::new();
    for i in 0..k {
      let tmp_clst = Cluster::new(i+1);
      self.clusters.push(tmp_clst);
    }
    self.choose_centroids(k);
  }

  pub fn print_clusters (&self) {
    for c in &self.clusters { c.print(); }
  }

  /* choose_centroids: handles random selection of initial cluster means */
  fn choose_centroids (&mut self, k: usize) {
    if self.data_set.len() < 1 { return; } //sanity check

    //random generator setup
    let domain = Range::new(1, self.data_set.len() + 1);
    let mut rng = rand::thread_rng();
    let mut randoms: Vec<usize> = Vec::new();

    loop { // until the desired k unique randoms are picked
      let random_selection = domain.ind_sample(&mut rng);
      if randoms.contains(&random_selection) { continue; }
      randoms.push(random_selection);
      if randoms.len() == k { break; }
    }
    /* now set the means for each cluster and the cluster ids for each random */
    for i in 0..k {
      let rand_id = randoms[i];
      let (data_index, err) = self.data_index(rand_id);
      if err { panic!("Error getting data index from id {}", rand_id); }
      let m = Mean {
        x: self.data_set[data_index].data[0],
        y: self.data_set[data_index].data[1]
      };
      self.clusters[i].set_mean(m);
      self.data_set[data_index].cluster = self.clusters[i].id;
    }
  }

  fn cluster_index (&self, id: usize) -> (usize, bool) {
    for i in 0..(self.clusters.len()) {
      if self.clusters[i].id == id { return (i, false); }
    }
    (0, true)
  }

  fn data_index (&self, id: usize) -> (usize, bool) {
    for i in 0..(self.data_set.len()) {
      if self.data_set[i].id == id { return (i, false); }
    }
    (0, true)
  }
}// impl Worker end

