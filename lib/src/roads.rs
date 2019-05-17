use crate::evac::EvacuationInfo;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
pub struct RoadNetwork {
    /// This map associates a node to a Vec containing the ids of
    /// the edges connected to this node.
    nodes: HashMap<u32, Vec<u32>>,
    edges: HashMap<u32, RoadEdge>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct RoadEdge {
    pub node1: u32,
    pub node2: u32,
    pub due_date: u64,
    pub length: f32,
    pub capacity: f32,
}

enum ParsingState {
    Section,
    Size,
    Road,
    End,
}

impl RoadNetwork {
    pub fn new() -> RoadNetwork {
        RoadNetwork {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    /// Read EvacuationInfo from a file.
    ///
    /// Parameters:
    /// * `filestr`: content of the file containing the data
    pub fn from_file(
        filestr: &str,
        evacinfo: &EvacuationInfo,
    ) -> Result<RoadNetwork, &'static str> {
        let mut parsing = ParsingState::Section;
        let mut node_count = -1i32;
        let mut key = 0;
        let mut result = RoadNetwork {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        };

        for line in filestr.lines() {
            let words: Vec<&str> = line.split(" ").collect();

            match parsing {
                ParsingState::Section => {
                    if line.starts_with("c [graph]") {
                        parsing = ParsingState::Size;
                    }
                }
                ParsingState::Size => {
                    node_count = words[1].parse::<i32>().unwrap();
                    parsing = if node_count == 0 {
                        ParsingState::End
                    } else {
                        ParsingState::Road
                    };
                }
                ParsingState::Road => {
                    if evacinfo.is_useful(
                        words[0].parse::<u32>().unwrap(),
                        words[1].parse::<u32>().unwrap(),
                    ) {
                        let edge = RoadEdge {
                            node1: words[0].parse::<u32>().unwrap(),
                            node2: words[1].parse::<u32>().unwrap(),
                            due_date: words[2].parse::<u64>().unwrap(),
                            length: words[3].parse::<f32>().unwrap(),
                            capacity: words[4].parse::<f32>().unwrap(),
                        };
                        result.add_edge_reference(edge.node1, key);
                        result.add_edge_reference(edge.node2, key);
                        result.edges.insert(key, edge);
                        key += 1;
                    }
                    node_count -= 1;

                    if node_count <= 0 {
                        parsing = ParsingState::End;
                    }
                }
                ParsingState::End => {}
            }
        }

        match parsing {
            ParsingState::End => Ok(result),
            _ => Err("Error while parsing"),
        }
    }

    fn add_edge_reference(&mut self, node: u32, edge: u32) {
        let vec = match self.nodes.entry(node) {
            Entry::Occupied(val) => val.into_mut(),
            Entry::Vacant(vac) => vac.insert(vec![]),
        };
        vec.push(edge);
    }

    pub fn add_road_edge(&mut self, key: u32, edge: RoadEdge) {
        self.add_edge_reference(edge.node1, key);
        self.add_edge_reference(edge.node2, key);
        self.edges.insert(key, edge);
    }
}
