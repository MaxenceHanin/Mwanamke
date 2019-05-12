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

#[derive(Clone, PartialEq, Debug)]
pub struct EvacuationSolution {
    name: String,
    nodes: Vec<SolutionNode>,
    pub valid: bool,
    pub goal_value: f32,
    /// compute time (expressed in seconds)
    compute_time: f32,
    method: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct SolutionNode {
    id: u32,
    evacuation_rate: f32,
    start_date: u32,
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

    pub fn is_useful(&self, node1: u32, node2: u32) -> bool {
        for node in &self.nodes {
            for i in 1..node.route.len() {
                if node1 == node.route[i] && node2 == node.route[i - 1]
                    || node1 == node.route[i - 1] && node2 == node.route[i]
                {
                    return true;
                }
            }
        }

        false
    }
}

impl EvacuationSolution {
    pub fn new(name: &str) -> EvacuationSolution {
        EvacuationSolution {
            name: String::from(name),
            nodes: vec![],
            valid: false,
            goal_value: 0.0,
            compute_time: 0.0,
            method: String::from("handmade v0.0.0"),
        }
    }

    pub fn from_file(filestr: &str) -> Result<EvacuationSolution, &str> {
        let mut result = EvacuationSolution::new("");
        let mut lines = filestr.lines();

        result.name = String::from(lines.next().unwrap());
        let node_count = lines.next().unwrap().parse::<u32>().unwrap();

        for _ in 0..node_count {
            let words: Vec<&str> = lines.next().unwrap().split(" ").collect();
            let node = SolutionNode {
                id: words[0].parse::<u32>().unwrap(),
                evacuation_rate: words[1].parse::<f32>().unwrap(),
                start_date: words[2].parse::<u32>().unwrap(),
            };
            result.nodes.push(node);
        }

        result.valid = lines.next().unwrap().starts_with("valid");
        result.goal_value = lines.next().unwrap().parse::<f32>().unwrap();
        result.compute_time = lines.next().unwrap().parse::<f32>().unwrap();
        result.method = String::from(lines.next().unwrap());

        Ok(result)
    }

    pub fn to_file(&self) -> String {
        let mut result = String::with_capacity(1000);
        result.push_str(self.name.as_str());
        result.push_str("\n");
        result.push_str(self.nodes.len().to_string().as_str());
        result.push_str("\n");

        for node in &self.nodes {
            result.push_str(node.id.to_string().as_str());
            result.push_str(" ");
            result.push_str(node.evacuation_rate.to_string().as_str());
            result.push_str(" ");
            result.push_str(node.start_date.to_string().as_str());
            result.push_str("\n");
        }

        if self.valid {
            result.push_str("valid\n");
        } else {
            result.push_str("invalid\n");
        }

        result.push_str(self.goal_value.to_string().as_str());
        result.push_str("\n");
        result.push_str(self.compute_time.to_string().as_str());
        result.push_str("\n");
        result.push_str(self.method.as_str());
        result.push_str("\n");

        result
    }

    pub fn add_node(&mut self, id: u32, evacuation_rate: f32, start_date: u32) {
        self.nodes.push(SolutionNode {
            id,
            evacuation_rate,
            start_date,
        });
    }
}
