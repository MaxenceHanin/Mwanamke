use crate::checker::EvacuationSolution;
use std::iter::Iterator;

#[derive(Clone, PartialEq, Debug)]
pub struct EvacuationInfo {
    pub safe_node: u32,
    nodes: Vec<EvacuationNode>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct EvacuationNode {
    pub id: u32,
    pub population: u32,
    pub max_rate: u32,
    pub route: Vec<u32>,
}

enum ParsingState {
    Section,
    Size,
    Node,
    End,
}

impl EvacuationInfo {
    pub fn new(safe_node: u32) -> EvacuationInfo {
        EvacuationInfo {
            safe_node,
            nodes: vec![],
        }
    }

    /// Read EvacuationInfo from a file.
    ///
    /// Parameters:
    /// * `filestr`: content of the file containing the data
    pub fn from_file(filestr: &str) -> Result<EvacuationInfo, &str> {
        let mut parsing = ParsingState::Section;
        let mut node_count = -1i32;
        let mut result = EvacuationInfo {
            safe_node: 0,
            nodes: vec![],
        };

        for line in filestr.lines() {
            let words: Vec<&str> = line.split(" ").collect();

            match parsing {
                ParsingState::Section => {
                    if line.starts_with("c [evacuation info]") {
                        parsing = ParsingState::Size;
                    }
                }
                ParsingState::Size => {
                    node_count = words[0].parse::<i32>().unwrap();
                    result.safe_node = words[1].parse::<u32>().unwrap();
                    parsing = if node_count == 0 {
                        ParsingState::End
                    } else {
                        ParsingState::Node
                    };
                }
                ParsingState::Node => {
                    let mut node = EvacuationNode {
                        id: words[0].parse::<u32>().unwrap(),
                        population: words[1].parse::<u32>().unwrap(),
                        max_rate: words[2].parse::<u32>().unwrap(),
                        route: vec![],
                    };

                    for word in &words[4..] {
                        node.route.push(word.parse::<u32>().unwrap());
                    }

                    result.nodes.push(node);

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

    pub fn add_node(&mut self, node: &EvacuationNode) {
        self.nodes.push(node.clone());
    }

    pub fn get_edge(&self, node1: u32, node2: u32) -> Option<(u32, u32)> {
        for node in &self.nodes {
            for i in 1..node.route.len() {
                if node1 == node.route[i] && node2 == node.route[i - 1]
                    || node1 == node.route[i - 1] && node2 == node.route[i]
                {
                    return Some((node.route[i - 1], node.route[i]));
                }
            }
        }

        None
    }
}
