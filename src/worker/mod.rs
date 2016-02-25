
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
      // TODO: implement this
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
      let id = randoms[i];
      let m = Mean {
        x: self.data_set[id].data[0],
        y: self.data_set[id].data[1]
      };
      self.clusters[i].set_mean(m);
      self.data_set[i].cluster = self.clusters[i].id;
    }
  }

}// impl Worker end

