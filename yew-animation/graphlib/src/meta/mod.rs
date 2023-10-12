use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

use rand::Rng;

use uuid::Uuid;

use serde::{Deserialize, Serialize};

use ordered_float::OrderedFloat;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Coordinate {
    pub x: OrderedFloat<f64>,
    pub y: OrderedFloat<f64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct NodeMetaData {
    pub coordinate: Coordinate,
}

impl NodeMetaData {
    pub fn new_random(range: Range<OrderedFloat<f64>>) -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(range.clone());
        let y = rng.gen_range(range.clone());

        Self {
            coordinate: Coordinate { x, y },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct EdgeMetaData {
    pub weight: OrderedFloat<f64>,
}

pub struct Line {
    pub from: (f64, f64),
    pub to: (f64, f64),
}
