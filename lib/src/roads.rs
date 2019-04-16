
use std::collections::HashMap;

struct RoadNetwork {
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
    fn from_file() {

    }
}