use crate::roads::RoadNetwork;
use crate::roads::RoadEdge;

pub fn next_node(tree:RoadNetwork, next : Option<RoadEdge>, time: u32, length: u32)-> u32{
    match next {
        // There is a child
        Some(x) => next_node(tree.clone(), tree.get_child_edge(x.child),time + length, x.length),
        // safe node reached
        None    => time,
    }
}
   
pub fn bound_inf(tree:RoadNetwork) -> u32{
    let mut result_final = 0;//u32::min_value();
    let mut result = u32::min_value();
    let mut _road : RoadNetwork;
    
    for node in tree.clone().evac_info.nodes {
        result = match tree.clone().get_child_edge(node.id){
                    Some(road) => next_node(tree.clone(),tree.get_child_edge(node.id),result,road.length),
                    None => u32::min_value(),
                };
        if result > result_final {
            result_final = result;
        }
    }
    return result_final;
}

#[cfg(test)]
mod tests {
    use crate::bounds::bound_inf;
    use crate::roads::RoadEdge;
    use crate::roads::RoadNetwork;
    use crate::evac::EvacuationNode;
    use crate::evac::EvacuationInfo;
    
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

            let inf = bound_inf(road_network);
            println!("Result found for test bound min: {}",inf);
            assert_eq!(inf, 92);
        }
}

pub fn bound_sup(tree:RoadNetwork) -> u32{
    let mut result_final = 0;//u32::min_value();
    let mut result = 0;
    let mut _road : RoadNetwork;
    
    for node in tree.clone().evac_info.nodes {
        if let Some(road) = tree.clone().get_child_edge(node.id){
            result = next_node(tree.clone(),tree.get_child_edge(node.id),result,road.length);
        }
        result_final = result_final + result;
    }
    return result_final;
}