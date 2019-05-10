
pub struct EvacuationInfo {
    safe_node: u32,
    nodes: Vec<EvacuationNode>,
}

struct EvacuationNode {
    id: u32,
    population: u32,
    max_rate: u32,
    route: Vec<u32>,
}

enum ParsingState {
    Section,
    Size,
    Node,
    End,
}

impl EvacuationInfo {

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
                    parsing = if node_count == 0 { ParsingState::End } else { ParsingState::Node };
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
                ParsingState::End => {},
            }
        }

        match parsing {
            ParsingState::End => Ok(result),
            _ => Err("Error while parsing"),
        }
    }

    pub fn is_useful(&self, node1:u32, node2:u32) -> bool{
        return true;
    }
}