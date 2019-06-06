use crate::checker::EvacuationSolution;
use crate::checker::SolutionNode;

/*--------------------- NEIGHBOURHOOD --------------------*/
//aim : find different types of neighbourhood

//fonction d'evaluation : this function calculate the time
//corresponding to the total evacuation


//incrementer/decrementer evac rate start date

/*
//evac rate neighbourhood : sort nodes by evac rates, big ones first
pub fn find_neighborhood(start: &Vec<SolutionNode>) -> Vec<SolutionNode>{
    let mut vec = Vec::new();
    let mut neighborhood = Vec::new();
    let mut _x : u32;
    for node in &start {
        vec.push(node.evacuation_rate);
    }
    vec.sort();
    while (vec.len()-1) >=0 {
        for node in &start{
            if node.evacuation_rate == vec[vec.len()-1] {
                neighborhood.push(node.clone());
            }
        }
    }
    return neighborhood;
}*/

pub fn neighbour_evac_rate(solution_start: EvacuationSolution) -> EvacuationSolution{
    let mut solution_neighborhood = solution_start.clone();
    solution_neighborhood.nodes = Vec::new();
    for node in solution_start.clone().nodes {
        let mut new_node = node.clone();
        new_node.evacuation_rate = new_node.evacuation_rate -1;
        solution_neighborhood.nodes.push(new_node);
    }
    return solution_neighborhood;
}
//random neighbourhood : select random nodes
/*pub fn random_neigh() -> u32{
    let vs = vec![0, 1, 2, 3, 4];
    println!("{:?}", vs.choose(&mut rand::thread_rng()));
}*/