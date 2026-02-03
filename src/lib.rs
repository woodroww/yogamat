//pub mod picking;
pub mod skeleton;
pub mod vector_ops;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use bincode::{Decode, Encode};
use skeleton::Joint;

#[derive(Debug, Serialize, Deserialize, Decode, Encode)]
pub struct Asana {
    pub asana_id: i32,
    pub pose_id: i32,
    pub sanskrit: String,
    pub english: String,
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize, Decode, Encode)]
pub struct AsanaData {
    pub asanas: Vec<Asana>,
    pub poses: HashMap<i32, Vec<Joint>>,
}

