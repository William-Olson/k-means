

pub struct Mean { pub x: f32, pub y: f32}

pub struct Cluster {
  pub id: usize,
  pub mean: Mean
}

impl Cluster {

    /* new: creates a new cluster */
    pub fn new(uid: usize) -> Cluster {
        let cluster = Cluster {
          id: uid,
          mean: Mean { x: -1.0, y: -1.0 }
        };
        cluster
    }
    /* set_mean: sets the cluster's mean property */
    pub fn set_mean(&mut self, m: Mean) {
      self.mean = m;
    }

    /* print: displays the current cluster object in the console */
    pub fn print (&self) {
      println!("{}: ( {}, {} )", self.id, self.mean.x, self.mean.y);
    }
}