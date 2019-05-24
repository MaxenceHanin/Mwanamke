pub mod checker;
pub mod evac;
pub mod roads;

#[cfg(test)]
mod tests {
    use crate::evac::{EvacuationInfo, EvacuationNode};
    use crate::roads::{RoadEdge, RoadNetwork};

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
    fn test_parsing_road_network() {
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
        let mut road_network = RoadNetwork::new();

        road_network.add_road_edge(
            0,
            RoadEdge {
                node1: 18,
                node2: 15,
                due_date: 51,
                length: 45,
                capacity: 100,
            },
        );
        road_network.add_road_edge(
            1,
            RoadEdge {
                node1: 15,
                node2: 5,
                due_date: 51,
                length: 92,
                capacity: 31,
            },
        );

        let info2 = RoadNetwork::from_file(
            "c [graph] blabla\n19 4\n12 13 51 46 49\n3 5 13 78 38\n18 15 51 45 100\n15 5 51 92 31\n", &evac_info
        );
        assert_eq!(road_network, info2.unwrap());
    }
}
