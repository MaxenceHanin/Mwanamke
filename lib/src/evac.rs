use std::iter::Iterator;

#[derive(Clone, PartialEq, Debug)]
pub struct EvacuationInfo {
    pub safe_node: u32,
    pub nodes: Vec<EvacuationNode>,
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

    pub fn dump(&self) {
        for n in &self.nodes {
            print!("{}", n.id);

            for i in &n.route {
                print!(" -> {}", i);
            }
            println!();
        }
    }

    pub fn add_node(&mut self, node: &EvacuationNode) {
        self.nodes.push(node.clone());
    }

    /// Returns EvacuationNode struct corresponding to the requested node.
    pub fn get_evacuation_data(&self, node_id: u32) -> Option<&EvacuationNode> {
        for node in &self.nodes {
            if node.id == node_id {
                return Some(node);
            }
        }
        None
    }

    pub fn get_edge(&self, node1: u32, node2: u32) -> Option<(u32, u32)> {
        for node in &self.nodes {
            for i in 0..node.route.len() {
                let parent_node = if i > 0 { node.route[i - 1] } else { node.id };

                if node1 == node.route[i] && node2 == parent_node
                    || node1 == parent_node && node2 == node.route[i]
                {
                    return Some((parent_node, node.route[i]));
                }
            }
        }

        None
    }
}
