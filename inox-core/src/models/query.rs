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
    pub tags_exclude: Vec<String>,
    pub omit_excluded: Exclude
}