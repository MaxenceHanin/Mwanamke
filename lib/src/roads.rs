
use std::collections::HashMap;

pub struct RoadNetwork {
    /// This map associates a node to a Vec containing the ids of
    /// the edges connected to this node.
    nodes: HashMap<u32, Vec<u32>>,
    edges: HashMap<u32, RoadEdge>,
}

struct RoadEdge {
    node1: u32,
    node2: u32,
    due_date: u64,
    length: f32,
    capacity: f32
}

impl RoadNetwork {
    pub fn from_file(filestr: &str) -> Result<RoadNetwork, &str> {
        Err("Not implemented")
    }
}