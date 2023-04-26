// src/models/bank.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Bank {
    pub id: i32,
    pub name: String,
    pub img: String,
}
