use std::fs::File;
use std::io::Read;

use lib_mwanamke::checker::EvacuationSolution;
use lib_mwanamke::evac::EvacuationInfo;
use lib_mwanamke::roads::RoadNetwork;

fn main() {
    let mut file = File::open("data/example.full").unwrap();

    let mut file_str = String::new();
    file.read_to_string(&mut file_str).unwrap();

    let evac_info = EvacuationInfo::from_file(&file_str).unwrap();
    let road_network = RoadNetwork::from_file(&file_str, evac_info).unwrap();

    file = File::open("data/example.sol").unwrap();

    file_str = String::new();
    file.read_to_string(&mut file_str).unwrap();

    let solution = EvacuationSolution::from_file(&file_str).unwrap();

    if solution.check(&road_network) {
        println!("Solution is correct!");
    } else {
        println!("Solution has problems in it");
    }
}
