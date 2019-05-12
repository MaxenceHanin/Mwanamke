pub mod evac;
pub mod roads;

#[cfg(test)]
mod tests {
    use crate::evac::{EvacuationInfo, EvacuationNode, EvacuationSolution};
    use crate::roads::RoadNetwork;

    #[test]
    fn test_parsing_evac_info() {
        let mut evac_info = EvacuationInfo::new(5);
        evac_info.add_node(&EvacuationNode {
            id: 4,
            population: 512,
            max_rate: 100,
            route: vec![18, 15, 5],
        });
        evac_info.add_node(&EvacuationNode {
            id: 6,
            population: 54,
            max_rate: 10,
            route: vec![19, 13, 5],
        });

        let info2 = EvacuationInfo::from_file(
            "c [evacuation info] blabla\n2 5\n4 512 100 3 18 15 5\n6 54 10 3 19 13 5\n",
        );
        assert_eq!(evac_info, info2.unwrap());

        let info3 = EvacuationInfo::from_file(
            "c [evacuation info] blabla\n2 5\n4 512 100 3 18 14 5\n6 54 10 3 19 13 5\n",
        );
        assert_ne!(evac_info, info3.unwrap());
    }

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
