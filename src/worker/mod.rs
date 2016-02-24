

mod cluster;
mod data_object;
use worker::cluster::*;
use worker::data_object::*;


struct Worker {
  clusters: Vec<Cluster>,
  dataSet: Vec<DataObject>,
  convergence: bool
}


impl Worker {
    pub fn new () -> Worker {
      let wkr = Worker {
        clusters: Vec::new(),
        dataSet: Vec::new(),
        convergence: false
      };
      wkr
    }
}