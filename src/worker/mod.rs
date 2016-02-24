

mod cluster;
mod data_object;
use worker::cluster::*;
use worker::data_object::*;


pub struct Worker {
  clusters: Vec<Cluster>,
  data_set: Vec<DataObject>,
  convergence: bool
}


impl Worker {
    pub fn new () -> Worker {
      let wkr = Worker {
        clusters: Vec::new(),
        data_set: Vec::new(),
        convergence: false
      };
      wkr
    }


    pub fn set_clusters (&mut self, k: usize) {
      self.clusters = Vec::new();
      for i in 0..k {
        let tmp_clst = Cluster::new(i+1);
        self.clusters.push(tmp_clst);
      }
      choose_centroids();
    }

    
}


fn choose_centroids () {

}