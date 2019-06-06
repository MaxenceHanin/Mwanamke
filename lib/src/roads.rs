use crate::evac::EvacuationInfo;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
pub struct RoadNetwork {
    pub evac_info: EvacuationInfo,
    /// This map associates a node to a Vec containing the ids of
    /// the edges connected to this node.
    nodes: HashMap<u32, Vec<u32>>,
    edges: HashMap<u32, RoadEdge>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct RoadEdge {
    pub parent: u32,
    pub child: u32,
    pub due_date: u64,
    pub length: u32,
    pub capacity: u32,
}

enum ParsingState {
    Section,
    Size,
    Road,
    End,
}

impl RoadNetwork {
    pub fn new(evac_info: EvacuationInfo) -> RoadNetwork {
        RoadNetwork {
            evac_info,
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
        evac_info: EvacuationInfo,
    ) -> Result<RoadNetwork, &'static str> {
        let mut parsing = ParsingState::Section;
        let mut node_count = -1i32;
        let mut key = 0;
        let mut result = RoadNetwork {
            evac_info,
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
                    if let Some((parent, child)) = result.evac_info.get_edge(
                        words[0].parse::<u32>().unwrap(),
                        words[1].parse::<u32>().unwrap(),
                    ) {
                        let edge = RoadEdge {
                            parent,
                            child,
                            due_date: words[2].parse::<u64>().unwrap(),
                            length: words[3].parse::<u32>().unwrap(),
                            capacity: words[4].parse::<u32>().unwrap(),
                        };
                        result.add_edge_reference(edge.parent, key);
                        result.add_edge_reference(edge.child, key);
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

    pub fn dump(&self) {
        self.evac_info.dump();

        for n in &self.nodes {
            print!("{}: ", n.0);

            for i in n.1 {
                print!("{} ", i);
            }
            println!();
        }
        println!("Edges:");

        for e in &self.edges {
            println!("* {} -> {}", e.1.parent, e.1.child);
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
        self.add_edge_reference(edge.parent, key);
        self.add_edge_reference(edge.child, key);
        self.edges.insert(key, edge);
    }

    pub fn get_child_edge(&self, node_id: u32) -> Option<RoadEdge> {
        if let Some(node) = self.nodes.get(&node_id) {
            for edge_id in node {
                let edge = self.edges.get(edge_id).unwrap();

                if edge.parent == node_id {
                    return Some(edge.clone());
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_get_chid_edge() {}
}
