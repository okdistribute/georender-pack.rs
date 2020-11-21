use crate::{PeerArea, PeerLine, PeerNode};
use desert::ToBytesLE;
use osm_is_area;
use std::collections::HashMap;

// Some convenience functions

pub fn node(id: u64, lon: f64, lat: f64, tags: Vec<(&str, &str)>) -> Vec<u8> {
    let node = PeerNode::new(id, lon, lat, &tags);
    return node.to_bytes_le().unwrap();
}

pub fn way(
    id: u64,
    tags: Vec<(&str, &str)>,
    refs: Vec<i64>,
    deps: &HashMap<i64, (f64, f64)>,
) -> Vec<u8> {
    let len = refs.len();
    if osm_is_area::way(&tags, &refs) {
        let positions = get_positions(&refs, &deps);
        let area = PeerArea::new(id, &tags, &positions);
        let buf = area.to_bytes_le().unwrap();
        return buf;
    } else if len > 1 {
        let positions = get_positions(&refs, &deps);
        let line = PeerLine::new(id, &tags, &positions);
        let buf = line.to_bytes_le().unwrap();
        return buf;
    } else {
        return vec![];
    }
}

fn get_positions(refs: &Vec<i64>, deps: &HashMap<i64, (f64, f64)>) -> Vec<(f64, f64)> {
    let mut positions = Vec::new();
    // positions
    for r in refs {
        let lon;
        let lat;
        match deps.get(r) {
            Some(dep) => {
                lon = dep.0;
                lat = dep.1;
                positions.push((lon, lat));
            }
            None => println!("Could not find dep for {}", &r),
        }
    }
    return positions;
}
