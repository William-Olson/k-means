//! A module for `DataObject`s.
//!
//! # Path
//!
//! worker/data_object.rs
//!
//! # Description
//!
//! Provides the DataObject struct and
//! methods for creating, printing and
//! comparing DataObjects.

use worker::cluster::Mean;

/// Holds data values, data id, & cluster id.
pub struct DataObject {
    pub id: usize,
    pub cluster: usize,
    pub data: Vec<f32>,
}

impl DataObject {
    /// Creates a new `DataObject` struct.
    pub fn new(uid: usize, d: &Vec<f32>) -> DataObject {
        let data_obj = DataObject {
            id: uid,
            cluster: 0,
            data: d.clone(),
        };
        data_obj
    }

    /// Displays the current `DataObject` to the console.
    pub fn print(&self) {
        println!("{} \t {:?} \t {}", self.id, self.data, self.cluster);
    }

    /// Calculates dissimilarity against the given operand.
    pub fn dist<T: DistOperand>(&self, operand: &T) -> f32 {
        operand.dist_cmp(&self)
    }
}

/// Allows comparing 2 `DataObject`s or
/// a `DataObject` with a `Mean` struct.
pub trait DistOperand {
    fn dist_cmp(&self, d: &DataObject) -> f32;
}

/// Calculates dissimilarity between
/// a `Mean` struct and a `DataObject`.
impl DistOperand for Mean {
    fn dist_cmp(&self, d: &DataObject) -> f32 {
        let diff1: f32 = d.data[0] - self.x;
        let diff2: f32 = d.data[1] - self.y;

        diff1.powi(2).sqrt() + diff2.powi(2).sqrt()
    }
}

/// Calculates dissimilarity between
/// two `DataObject`s.
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
