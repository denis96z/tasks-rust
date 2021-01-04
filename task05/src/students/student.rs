use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Serialize, Deserialize)]
pub struct Student {
    #[serde(rename = "first_name")]
    pub first_name: String,

    #[serde(rename = "last_name")]
    pub last_name: String,

    #[serde(rename = "age")]
    pub age: Option<u8>,

    #[serde(rename = "gender")]
    pub gender: Option<Gender>,

    #[serde(rename = "grades", default)]
    pub grades: HashMap<String, Grade>,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Gender {
    Male = 1,
    Female = 2,
    Other = 0,
}

pub type Grade = u8;
