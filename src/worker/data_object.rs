

pub struct DataObject {
  id: usize,
  cluster: usize,
  data: Vec<f32>
}

impl DataObject {
  pub fn new(uid: usize, d: &Vec<f32>) -> DataObject {
      let dataObj = DataObject {
        id: uid,
        cluster: 0,
        data: d.clone()
      };
      dataObj
  }
}