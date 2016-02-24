

pub struct Mean {x: f32, y: f32}

pub struct Cluster {
  id: usize,
  mean: Mean
}

impl Cluster {
    pub fn new(uid: usize) -> Cluster {
        let cluster = Cluster {
          id: uid,
          mean: Mean { x: -1.0, y: -1.0 }
        };
        cluster
    }
}