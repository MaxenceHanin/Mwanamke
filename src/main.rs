use std::fs::File;
use std::io::Read;

use lib_mwanamke::evac::EvacuationInfo;
use lib_mwanamke::roads::RoadNetwork;

fn main() {
    let mut file = File::open("data/sparse_10_30_3_1.full").unwrap();

    let mut file_str = String::new();
    file.read_to_string(&mut file_str).unwrap();

    let evac_info = EvacuationInfo::from_file(&file_str).unwrap();
    let road_network = RoadNetwork::from_file(&file_str).unwrap();
}
