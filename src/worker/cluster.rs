//! A module for creating `Cluster`s.
//!
//! # Path
//!
//! worker/cluster.rs
//!
//! # Description
//!
//! Provides the Cluster struct and
//! methods for creation, printing, and
//! setting the mean of the Cluster.


pub struct Mean { pub x: f32, pub y: f32}

pub struct Cluster {
  pub id: usize,
  pub mean: Mean
}

impl Cluster {

    /// Creates a new cluster.
    pub fn new(uid: usize) -> Cluster {
        let cluster = Cluster {
          id: uid,
          mean: Mean { x: -1.0, y: -1.0 }
        };
        cluster
    }
    /// Sets the cluster's mean property.
    pub fn set_mean(&mut self, m: Mean) {
      self.mean = m;
    }

    /// Displays the current cluster object in the console.
    pub fn print (&self) {
      println!("{}: ( {}, {} )", self.id, self.mean.x, self.mean.y);
    }
}
