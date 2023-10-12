use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

use rand::Rng;

use uuid::Uuid;

use serde::{Deserialize, Serialize};

use ordered_float::OrderedFloat;

use crate::Graph;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
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
    fn crossing_any(&self, others: Vec<&Self>) -> bool {
        others.iter().any(|other| self.crossing(other))
    }
    fn crossing_all(&self, others: Vec<&Self>) -> bool {
        others.iter().all(|other| self.crossing(other))
    }
}

impl Graph {
    pub fn get_edge_crossings(&self) -> Vec<((Uuid, Uuid), (Uuid, Uuid))> {
        let mut result = vec![];
        for edge in &self.edges {
            let line = self.get_line_of_edge(edge).unwrap();
            for inner_edge in &self.edges {
                if inner_edge == edge {
                    continue;
                }
                let inner_line = self.get_line_of_edge(inner_edge).unwrap();
                if !line.crossing(&inner_line) {
                    continue;
                }
                result.push((
                    (edge.incoming, edge.outgoing),
                    (inner_edge.incoming, inner_edge.outgoing),
                ));
            }
        }
        result
    }
}
