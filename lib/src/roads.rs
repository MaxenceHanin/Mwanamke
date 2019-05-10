
use std::collections::HashMap;
use crate::evac::EvacuationInfo;

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

enum ParsingState {
    Section,
    Size,
    Road,
    End,
}

impl RoadNetwork {
    
        /// Read EvacuationInfo from a file.
    ///
    /// Parameters:
    /// * `filestr`: content of the file containing the data
    pub fn from_file(filestr: &str, evacinfo:&EvacuationInfo) -> Result<RoadNetwork, &str> {
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
                    parsing = if node_count == 0 { ParsingState::End } else { ParsingState::Road };
                }
                ParsingState::Road => {
                    if evacinfo.is_useful(words[0].parse::<u32>().unwrap(),words[1].parse::<u32>().unwrap()) {
                        let mut edges = RoadEdge {
                            node1:words[0].parse::<u32>().unwrap(),
                            node2:words[1].parse::<u32>().unwrap(),
                            due_date:words[2].parse::<u64>().unwrap(),
                            length:words[3].parse::<f32>().unwrap(),
                            capacity:words[4].parse::<f32>().unwrap(),
                        };
                    result.edges.insert(key,edges);
                    result.nodes.insert(key,words[0]);
                    key+=1;
                    }
                    node_count -= 1;

                    if node_count <= 0 {
                        parsing = ParsingState::End;
                    }
                }
                ParsingState::End => {},
            }
        }

        match parsing {
            ParsingState::End => Ok(result),
            _ => Err("Error while parsing"),
        }
    }
}