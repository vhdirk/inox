use strum::Display;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Display, Debug, Clone)]
pub enum Sort {
    OldestFirst,
    NewestFirst,
    MessageID,
    Unsorted
}

#[derive(Serialize, Deserialize, Display, Debug, Clone)]
pub enum Exclude {
    Flag,
    True,
    False,
    All
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Query {
    pub query: String,
    pub sort: Sort,
    pub exclude_tags: Vec<String>,
    pub exclude: Exclude,
    pub limit: Option<u32>,
    pub offset: Option<u32>
}


