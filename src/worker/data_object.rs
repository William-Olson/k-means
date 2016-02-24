

pub struct DataObject {
  pub id: usize,
  pub cluster: usize,
  pub data: Vec<f32>
}

impl DataObject {
  /* new: creates a new data_object struct */
  pub fn new(uid: usize, d: &Vec<f32>) -> DataObject {
      let data_obj = DataObject {
        id: uid,
        cluster: 0,
        data: d.clone()
      };
      data_obj
  }
  /* print: displays the current data_object to the console */
  pub fn print(&self) {
    println!("{} \t {:?} \t {}", self.id, self.data, self.cluster);
  }
}
