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
    pub from: Coordinate,
    pub to: Coordinate,
}

impl Line {
    fn crossing(&self, other: &Self) -> bool {
        // Calculate the direction vectors of the lines
        let dx1 = self.to.x - self.from.x;
        let dy1 = self.to.y - self.from.y;
        let dx2 = other.to.x - other.from.x;
        let dy2 = other.to.y - other.from.y;

        // Calculate determinants
        let det = dx1 * dy2 - dx2 * dy1;

        // Check if the lines are not parallel and intersect
        if det.abs() < 1e-10 {
            return false;
        }

        // Calculate the parameters for the intersection point
        let t1 = ((other.from.x - self.from.x) * dy2 - (other.from.y - self.from.y) * dx2) / det;
        let t2 = ((other.from.x - self.from.x) * dy1 - (other.from.y - self.from.y) * dx1) / det;

        // Check if the intersection point is within the lines
        (OrderedFloat(0.0) <= t1 && t1 <= OrderedFloat(1.0))
            && (OrderedFloat(0.0) <= t2 && t2 <= OrderedFloat(1.0))
    }
}
