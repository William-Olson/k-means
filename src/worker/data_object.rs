/*
* Program:     k-means (hw2)
* Author:      Will Olson
* Date:        2/25/2016
*
* File: worker/data_object.rs
*/

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

  pub fn dist(&self, other: &DataObject) -> f32 {
    let mut ds: f32 = 0.0;
    for i in 0..(self.data.len()) {
      //rust has math methods on some primitives:
      //  https://doc.rust-lang.org/std/primitive.f32.html#method.sqrt
      //  https://doc.rust-lang.org/std/primitive.f32.html#method.exp2
      let diff = self.data[i] - other.data[i];
      ds += (diff.exp2()).sqrt();
    }
    ds
  }
}
