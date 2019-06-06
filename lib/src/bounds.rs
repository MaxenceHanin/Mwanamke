use crate::roads::RoadEdge;
use crate::roads::RoadNetwork;
use crate::checker::EvacuationSolution;
use crate::evac::EvacuationNode;
use std::time::Instant;

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

//find max evac_rate for each evac node
pub fn vec_node(tree: RoadNetwork, mut vec:Vec<u32>, node: EvacuationNode ) -> Vec<u32> {
    //println!("time: {}", time);
    let mut next_node = node.clone();
    match tree.clone().get_child_edge(node.id) {
        // There is a child
        Some(x) => {
            vec.push(x.length);
            for node in tree.clone().evac_info.nodes {
                if x.child == node.id{
                    next_node = node;
                }
            }

            vec_node(
                tree.clone(),
                vec,
                next_node,
            )
        },
        // safe node reached
        None => vec,
    }
}
pub fn find_max_evac_rate(tree: RoadNetwork, node: EvacuationNode) -> (u32,u32) {
    let mut tot = 0;
    let mut res = Vec::new();
    let mut vec =vec_node(tree.clone(), res, node);
    vec.sort();
    let mut vec_len =vec.len()-1;
    while vec_len >=0 {
        tot = tot+vec[vec_len];
        vec_len = vec_len -1;
    }
    return (vec[0],tot);
}

//bound inf but returning EvacuationSolution
pub fn bound_inf_evac_sol(tree: RoadNetwork) -> EvacuationSolution {

    let earlier = Instant::now();
    let mut nb =0;
    let mut start_evac =0;
    let mut max_ev_rate =(0,0);
    let mut evac_solution = EvacuationSolution::new("solution");
    evac_solution.valid = true;
    evac_solution.goal_value = bound_inf(tree.clone()) as f32;

    for node in tree.clone().evac_info.nodes {
        max_ev_rate =find_max_evac_rate(tree.clone(), node.clone());
        nb = (node.population / max_ev_rate.1)+1;
        evac_solution.add_node(node.id, max_ev_rate.0+nb, start_evac);
        start_evac = start_evac + max_ev_rate.0+nb;
    }
    let now = Instant::now();
    evac_solution.compute_time = now.duration_since(earlier).as_secs() as f32;
    return evac_solution;
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
