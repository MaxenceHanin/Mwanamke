use crate::roads::RoadNetwork;
use std::iter::Iterator;

#[derive(Clone, PartialEq, Debug)]
pub struct EvacuationSolution {
    name: String,
    pub nodes: Vec<SolutionNode>,
    pub valid: bool,
    pub goal_value: f32,
    /// compute time (expressed in seconds)
    pub compute_time: f32,
    method: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct SolutionNode {
    id: u32,
    pub evacuation_rate: u32,
    pub start_date: u32,
}

#[derive(Clone)]
struct NodeCheck {
    start_id: u32,
    end_id: u32,
    start_date: u32,
    end_date: u32,
    start_rate: u32,
    end_rate: u32,
    max_rate: u32,
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
                evacuation_rate: words[1].parse::<u32>().unwrap(),
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

    pub fn add_node(&mut self, id: u32, evacuation_rate: u32, start_date: u32) {
        self.nodes.push(SolutionNode {
            id,
            evacuation_rate,
            start_date,
        });
    }

    pub fn check(&self, roads: &RoadNetwork) -> bool {
        let mut all_nodes: Vec<NodeCheck> = vec![];

        // Initialization
        for sol_node in &self.nodes {
            let edge = roads.get_child_edge(sol_node.id).unwrap();
            let evac_node = roads.evac_info.get_evacuation_data(sol_node.id).unwrap();

            if sol_node.evacuation_rate > edge.capacity {
                return false;
            }
            let duration = (evac_node.population - 1) / sol_node.evacuation_rate + 1;
            let end_rate = evac_node.population % sol_node.evacuation_rate;

            all_nodes.push(NodeCheck {
                start_id: sol_node.id,
                start_date: sol_node.start_date,
                start_rate: sol_node.evacuation_rate,
                end_id: edge.child,
                end_date: sol_node.start_date + duration,
                end_rate,
                max_rate: edge.capacity,
            });
        }

        let mut current_nodes: Vec<NodeCheck> = all_nodes.to_vec();

        while !current_nodes.is_empty() {
            let old_nodes = current_nodes;
            current_nodes = vec![];

            // Create the new nodes
            for old_node in &old_nodes {
                if old_node.end_id != roads.evac_info.safe_node {
                    let old_edge = roads.get_child_edge(old_node.start_id).unwrap();
                    let edge = roads.get_child_edge(old_node.end_id).unwrap();

                    current_nodes.push(NodeCheck {
                        start_id: edge.parent,
                        start_date: old_node.start_date + old_edge.length,
                        start_rate: old_node.start_rate,
                        end_id: edge.child,
                        end_date: old_node.end_date + old_edge.length,
                        end_rate: old_node.end_rate,
                        max_rate: edge.capacity,
                    })
                }
            }

            // Check if they work with the old list
            for new_node in &current_nodes {
                for t in new_node.start_date..(new_node.end_date + 1) {
                    let mut count = 0;

                    for old_node in &all_nodes {
                        if old_node.start_id == new_node.start_id {
                            if old_node.end_date == t {
                                count += old_node.end_rate;
                            } else if old_node.start_rate <= t && t < old_node.end_date {
                                count += old_node.start_rate;
                            }
                        }
                    }

                    if count > new_node.max_rate {
                        return false;
                    }
                }
                all_nodes.push(new_node.clone());
            }
        }

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
        evac_solution.add_node(5, 10, 4);

        assert_eq!(
            evac_solution,
            EvacuationSolution::from_file(evac_solution.to_file().as_str()).unwrap()
        );
    }
}
