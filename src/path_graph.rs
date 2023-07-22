use std::collections::HashMap;

use bit_vec::BitVec;

pub struct PathGraph {
    pub lnz: Vec<char>,
    pub nws: BitVec,
    pub succ_hash: SuccHash,
    pub paths_nodes: Vec<BitVec>,
    pub paths_number: usize,
    pub nodes_id_pos: Vec<u64>,
}

impl PathGraph {
    pub fn new() -> PathGraph {
        PathGraph {
            lnz: vec![],
            nws: BitVec::new(),
            succ_hash: SuccHash::new(),
            paths_nodes: vec![],

            paths_number: 0,
            nodes_id_pos: vec![],
        }
    }

    pub fn build(
        lnz: Vec<char>,
        nws: BitVec,
        succ_hash: SuccHash,
        paths_nodes: Vec<BitVec>,
        paths_number: usize,
        nodes_id_pos: Vec<u64>,
    ) -> PathGraph {
        PathGraph {
            lnz,
            nws,
            succ_hash,
            paths_nodes,
            paths_number,
            nodes_id_pos,
        }
    }

    pub fn to_string(self) {
        println!("Linearization:");
        println!("{:?}", self.lnz);
        println!();

        println!("Nodes with succs:");
        println!("{:?}", self.nws);
        println!();

        println!("succs hash:");
        println!("{:?}", self.succ_hash);
        println!();

        println!("Paths of nodes:");
        println!("{:?}", self.paths_nodes);
        println!();

        println!("nodes_id pos:");
        println!("{:?}", self.nodes_id_pos);
        println!();

        println!("Number of paths: {}", self.paths_number);
    }
}

impl Default for PathGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct SuccHash {
    successor: HashMap<usize, HashMap<usize, BitVec>>,
}

impl SuccHash {
    pub fn new() -> SuccHash {
        SuccHash {
            successor: HashMap::new(),
        }
    }

    pub fn get_succs_and_paths(&self, curr_node: usize) -> Vec<(usize, BitVec)> {
        let succs = self.successor.get(&curr_node).unwrap();
        let mut succs_paths = Vec::new();
        for (succ_pos, succ_paths) in succs.iter() {
            succs_paths.push((*succ_pos, succ_paths.clone()));
        }
        succs_paths
    }

    pub fn set_succs_and_paths(
        &mut self,
        curr_node: usize,
        succ_pos: usize,
        path_id: usize,
        paths_number: usize,
    ) {
        if self.successor.get(&curr_node).is_none() {
            self.successor.insert(curr_node, HashMap::new());
        }

        if self
            .successor
            .get(&curr_node)
            .unwrap()
            .get(&succ_pos)
            .is_none()
        {
            self.successor
                .get_mut(&curr_node)
                .unwrap()
                .insert(succ_pos, BitVec::from_elem(paths_number, false));
        }
        self.successor
            .get_mut(&curr_node)
            .unwrap()
            .get_mut(&succ_pos)
            .unwrap()
            .set(path_id, true);
    }
}

impl Default for SuccHash {
    fn default() -> Self {
        Self::new()
    }
}
