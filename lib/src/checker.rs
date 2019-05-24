use crate::evac::EvacuationInfo;
use std::collections::HashMap;
use std::iter::Iterator;

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

struct NodeCheck {
    start_date: u32,
    end_date: u32,
    start_rate: u32,
    end_rate: u32,
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

    pub fn check(&self, evac_info: &EvacuationInfo) -> bool {
        let mut nodes: HashMap<u32, NodeCheck> = HashMap::new();
        let mut current_nodes: Vec<u32> = vec![];

        // Initialization
        true
    }
}

#[cfg(test)]
mod tests {
    use super::EvacuationSolution;

    #[test]
    fn test_parsing_evac_solution() {
        let mut evac_solution = EvacuationSolution::new("solution");
        evac_solution.valid = true;
        evac_solution.goal_value = 48.0;
        evac_solution.add_node(5, 10.0, 4);

        assert_eq!(
            evac_solution,
            EvacuationSolution::from_file(evac_solution.to_file().as_str()).unwrap()
        );
    }

}
