use crate::roads::RoadEdge;
use crate::roads::RoadNetwork;

//recursive func to find all the child, add their length and return the total
pub fn next_node(tree: RoadNetwork, next: Option<RoadEdge>, time: u32, _length: u32) -> u32 {
    //println!("time: {}", time);
    match next {
        // There is a child
        Some(x) => next_node(
            tree.clone(),
            tree.get_child_edge(x.child),
            time + x.length,
            x.length,
        ),
        // safe node reached
        None => time,
    }
}
//bound inf is the max of node's evacuation time, for each node, if they are conidered as alone
pub fn bound_inf(tree: RoadNetwork) -> u32 {
    let mut result_final = 0; //u32::min_value();
    let mut result = 0;
    let mut _road: RoadNetwork;

    for node in tree.clone().evac_info.nodes {
        result = match tree.clone().get_child_edge(node.id) {
            Some(road) => next_node(tree.clone(), tree.get_child_edge(node.id), 0, road.length),
            None => result,
        };
        //println!("res: {}", result);
        if result > result_final {
            result_final = result;
        }
    }
    return result_final;
}

//bound sup is the total of node's evacuation time, if they are conidered as alone
pub fn bound_sup(tree: RoadNetwork) -> u32 {
    let mut result_final = 0; //u32::min_value();
    let mut result = 0;
    let mut _road: RoadNetwork;

    for node in tree.clone().evac_info.nodes {
        result = match tree.clone().get_child_edge(node.id) {
            Some(road) => next_node(tree.clone(), tree.get_child_edge(node.id), 0, road.length),
            None => result,
        };
        //println!("res: {}", result);
        result_final = result_final + result;
    }
    return result_final;
}

#[cfg(test)]
mod tests {
    use crate::bounds::bound_inf;
    use crate::bounds::bound_sup;
    use crate::evac::EvacuationInfo;
    use crate::evac::EvacuationNode;
    use crate::roads::RoadEdge;
    use crate::roads::RoadNetwork;

    #[test]
    fn test_bound_inf() {
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
        let mut road_network = RoadNetwork::new(evac_info.clone());

        road_network.add_road_edge(
            4,
            RoadEdge {
                parent: 4,
                child: 18,
                due_date: 26,
                length: 54,
                capacity: 72,
            },
        );

        road_network.add_road_edge(
            0,
            RoadEdge {
                parent: 18,
                child: 15,
                due_date: 51,
                length: 45,
                capacity: 100,
            },
        );
        road_network.add_road_edge(
            1,
            RoadEdge {
                parent: 15,
                child: 5,
                due_date: 51,
                length: 92,
                capacity: 31,
            },
        );

        road_network.add_road_edge(
            5,
            RoadEdge {
                parent: 6,
                child: 19,
                due_date: 26,
                length: 42,
                capacity: 55,
            },
        );

        road_network.add_road_edge(
            2,
            RoadEdge {
                parent: 19,
                child: 13,
                due_date: 31,
                length: 18,
                capacity: 80,
            },
        );
        road_network.add_road_edge(
            3,
            RoadEdge {
                parent: 13,
                child: 5,
                due_date: 26,
                length: 29,
                capacity: 45,
            },
        );
        let inf = bound_inf(road_network);
        println!("Result found for test bound min: {}", inf);
        assert_eq!(inf, 191);
    }

    #[test]
    fn test_bound_sup() {
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
        let mut road_network = RoadNetwork::new(evac_info.clone());

        road_network.add_road_edge(
            4,
            RoadEdge {
                parent: 4,
                child: 18,
                due_date: 26,
                length: 54,
                capacity: 72,
            },
        );

        road_network.add_road_edge(
            0,
            RoadEdge {
                parent: 18,
                child: 15,
                due_date: 51,
                length: 45,
                capacity: 100,
            },
        );
        road_network.add_road_edge(
            1,
            RoadEdge {
                parent: 15,
                child: 5,
                due_date: 51,
                length: 92,
                capacity: 31,
            },
        );

        road_network.add_road_edge(
            5,
            RoadEdge {
                parent: 6,
                child: 19,
                due_date: 26,
                length: 42,
                capacity: 55,
            },
        );

        road_network.add_road_edge(
            2,
            RoadEdge {
                parent: 19,
                child: 13,
                due_date: 31,
                length: 18,
                capacity: 80,
            },
        );
        road_network.add_road_edge(
            3,
            RoadEdge {
                parent: 13,
                child: 5,
                due_date: 26,
                length: 29,
                capacity: 45,
            },
        );

        let sup = bound_sup(road_network);
        println!("Result found for test bound sup: {}", sup);
        assert_eq!(sup, 280);
    }
}
