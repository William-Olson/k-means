/*
 * Program:     k-means (hw2)
 * Author:      Will Olson
 * Date:        2/25/2016
 *
 * File: worker/data_object.rs
*/

use worker::cluster::Mean;

/* DataObject: 
 *   holds data values, data id, & cluster id 
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

  pub fn dist<T: DistOperand>(&self, operand: &T) -> f32 {
    operand.dist_cmp(&self)
  }

}


/* trait: DistOperand 
 *   allows comparing 2 DataObjects or
 *   a DataObject with a Mean struct
 *
 * side_note:
 *  Rust has math methods attached to the f32 primitives.
 *  for details see:
 *    https://doc.rust-lang.org/std/primitive.f32.html#method.sqrt
 *    https://doc.rust-lang.org/std/primitive.f32.html#method.powi
*/
trait DistOperand {
  fn dist_cmp(&self, d: &DataObject) -> f32;
}

impl DistOperand for Mean {
  fn dist_cmp(&self, d: &DataObject) -> f32 {
     let diff1: f32 = d.data[0] - self.x;
     let diff2: f32 = d.data[1] - self.y;

     ( diff1.powi(2).sqrt() + diff2.powi(2).sqrt() )
  }
}
//more general
impl DistOperand for DataObject {
  fn dist_cmp(&self, d: &DataObject) -> f32 {
    let mut ds: f32 = 0.0;
    for i in 0..(d.data.len()) {
      let diff = d.data[i] - self.data[i];
      ds += (diff.powi(2)).sqrt();
    }
    ds
  }
}

