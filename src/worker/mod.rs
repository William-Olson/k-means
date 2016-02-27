//! A module for interacting with `Worker`s.
//!
//! # Path
//!
//! worker/mod.rs
//!
//! # Description
//!
//!




// imports
extern crate rand;
use worker::rand::distributions::{IndependentSample, Range};

// std libs
use std::f32::INFINITY;

// local modules
mod cluster;
mod data_object;
use worker::cluster::*;
use worker::data_object::*;


/// Holds cluster & data `Vec`s, and
/// a convergence boolean for determining
/// if the k-means algorithm is done.
pub struct Worker {
  clusters: Vec<Cluster>,
  data_set: Vec<DataObject>,
  convergence: bool
}


// methods for Worker
impl Worker {

  /// Creates a new `Worker` struct.
  pub fn new () -> Worker {
    let wkr = Worker {
      clusters: Vec::new(),
      data_set: Vec::new(),
      convergence: false
    };
    wkr
  }

  /// Parses the given `blob` into a `DataObject`s 
  /// and adds them to the `Worker`s `data_set`.
  pub fn set_data (&mut self, blob: &String) {
    let ref_data: Vec<&str> = blob.split("\n").collect(); //split by line
    let mut id = 1;
    for line in &ref_data { // split by whitespace
      if line.is_empty() == false {
        // parse vals, create object & add to vector
        self.data_set.push(
            DataObject::new(id, &( line.split_whitespace().map(
                         |val| val.parse::<f32>().unwrap()).collect() )
            ) // might want to check for parse errors instead of unwrap()
        );
        id += 1;
      }
    }
  }

  /// Prints the distances from the `DataObject` with the 
  /// given id against all other `DataObjects` in `data_set`.
  pub fn print_data_dists(&self, id: usize) {
    let mut res: Vec<(usize, f32)> = Vec::new();
    let (index, err) = self.data_index(id);
    if err { return; }
    for d in &self.data_set {
      res.push((d.id, (self.data_set[index]).dist(d)));
    }
    // find the min distance
    let mut min_dist: (usize, f32) = (0, INFINITY);
    for tpl in &res {
      if (tpl.1).is_nan() == false {
        if tpl.1 <  min_dist.1 {
          min_dist = tpl.clone();
        }
      }
    }
    println!("{:?}", res);
    println!("min_dist: {:?}", min_dist);
  }

  /// Prints dissimilarity information to the console
  /// about the `DataObject` with the given id against
  /// all the current cluster centroids in `clusters`.
  pub fn print_mean_dists(&self, id: usize) {
    let mut res: Vec<(usize, f32)> = Vec::new();
    let (index, err) = self.data_index(id);
    if err { return; }
    for c in &self.clusters {
      res.push((c.id, (self.data_set[index]).dist(&(c.mean))));
    }
    // find the minimum distance between objects
    let mut min_dist: (usize, f32) = (0, INFINITY);
    for tpl in &res {
      if (tpl.1).is_nan() == false {
        if tpl.1 <  min_dist.1 {
          min_dist = tpl.clone();
        }
      }
    }
    println!("{:?}", res);
    println!("min_dist: {:?}", min_dist);
  }

  /// Shows the `data_set` values in the console.
  pub fn print_data (&self) {
    for d in &self.data_set { d.print(); }
  }

  /// Creates a vector of `Cluster`s and assigns
  /// it to the `Worker`s `clusters` property.
  pub fn set_clusters (&mut self, k: usize) {
    self.clusters = Vec::new();
    for i in 0..k {
      let tmp_clst = Cluster::new(i+1);
      self.clusters.push(tmp_clst);
    }
    self.choose_centroids(k);
  }

  /// Shows the `clusters` vector in the console.
  pub fn print_clusters (&self) {
    for c in &self.clusters { c.print(); }
  }

  /// Handles random selection of initial cluster means.
  fn choose_centroids (&mut self, k: usize) {

    // random generator setup
    let domain = Range::new(1, self.data_set.len() + 1);
    let mut rng = rand::thread_rng();
    let mut randoms: Vec<usize> = Vec::new();

    loop { // until the desired k unique randoms are picked
      let random_selection = domain.ind_sample(&mut rng);
      if randoms.contains(&random_selection) { continue; }
      randoms.push(random_selection);
      if randoms.len() == k { break; }
    }
    // set means for each cluster and the cluster ids for each random
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

  /// Retrieves the index of the `Cluster` with the given id.
  fn cluster_index (&self, id: usize) -> (usize, bool) {
    for i in 0..(self.clusters.len()) {
      if self.clusters[i].id == id { return (i, false); }
    }
    (0, true)
  }


  /// Retrieves the index of the `ObjectObject` with the given id.
  fn data_index (&self, id: usize) -> (usize, bool) {
    for i in 0..(self.data_set.len()) {
      if self.data_set[i].id == id { return (i, false); }
    }
    (0, true)
  }

  /// Calculates new centroid values for all `clusters`.
  fn calc_means (&mut self) -> bool {
    //TODO: implement this
    true
  }

  /// Runs the k-means algorithm on `data_set`.
  pub fn run (&mut self) {
    //TODO: implement this
  }

  /// Creates and returns a String containing the
  /// `data_set` data & cluster id associations.
  pub fn results_to_string (&self) -> String {
    let mut res = String::from("data \t     \t cluster\n");
    for o in &self.data_set {
      res.push_str(&(o.data[0].to_string()));
      res.push_str(" \t ");
      res.push_str(&(o.data[1].to_string()));
      res.push_str(" \t ");
      res.push_str(&(o.cluster.to_string()));
      res.push('\n');
    }
    res
  }

}// impl Worker end

