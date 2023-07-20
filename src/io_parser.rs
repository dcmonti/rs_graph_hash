use bit_vec::BitVec;
use gfa::{gfa::*, parser::GFAParser};
use handlegraph::{
    handle::{Handle, NodeId},
    handlegraph::HandleGraph,
    hashgraph::HashGraph,
};
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

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

pub fn read_graph_w_path(file_path: &str, is_reversed: bool) -> PathGraph {
    let parser = GFAParser::new();
    let gfa: GFA<usize, ()> = parser.parse_file(file_path).unwrap();

    let graph: HashGraph = HashGraph::from_gfa(&gfa);
    create_path_graph(&graph, is_reversed)
}

pub fn create_path_graph(graph: &HashGraph, is_reversed: bool) -> PathGraph {
    let mut sorted_handles = graph.handles_iter().collect::<Vec<Handle>>();
    sorted_handles.sort();

    if is_reversed {
        sorted_handles.reverse();
        sorted_handles = sorted_handles
            .iter()
            .map(|h| h.flip())
            .collect::<Vec<Handle>>();
    }
    
    //create graph linearization
    let mut last_index = 1;
    let mut visited_node: HashMap<NodeId, i32> = HashMap::new();
    let mut linearization: Vec<char> = vec!['$'];
    let mut handles_id_position = HashMap::new();
    let mut nodes_id_pos = Vec::new();
    nodes_id_pos.push(0);
    for handle in sorted_handles.iter() {
        let start_position = last_index;
        for ch in graph.sequence(*handle) {
            linearization.push(ch as char);
            nodes_id_pos.push(handle.id().into());
            last_index += 1;
        }
        let end_position = last_index - 1;
        visited_node.insert(handle.id(), end_position);
        handles_id_position.insert(handle.id(), (start_position, end_position));
    }
    linearization.push('F');
    nodes_id_pos.push(0);

    //create nws, succ_hash,nodes paths and
    let mut nodes_with_succ = BitVec::from_elem(linearization.len(), false);
    let mut succ_hash_struct = SuccHash::new();

    let paths_set = &graph.paths;
    let mut paths = Vec::new();
    for (_id, path) in paths_set.iter() {
        paths.push(path)
    }
    for (id, path) in paths_set.iter() {
        paths[*id as usize] = path
    }

    
    let paths_number = paths_set.keys().len();

    let mut paths_nodes = vec![BitVec::from_elem(paths_number, false); linearization.len()];

    paths_nodes[0] = BitVec::from_elem(paths_number, true);

    for (path_id, path) in paths.iter().enumerate() {
        let path_nodes = if is_reversed {
            path.nodes.iter().rev().collect::<Vec<&Handle>>()
        } else {
            path.nodes.iter().collect::<Vec<&Handle>>()
        };

        for (pos, handle) in path_nodes.iter().enumerate() {
            let (handle_start, handle_end) = handles_id_position.get(&handle.id()).unwrap();
            let handle_start = *handle_start as usize;
            let handle_end = *handle_end as usize;

            for idx in handle_start..=handle_end {
                paths_nodes[idx].set(path_id as usize, true);
            }

            if !nodes_with_succ[handle_end] {
                nodes_with_succ.set(handle_end, true);
            }

            if pos == 0 {
                succ_hash_struct.set_succs_and_paths(0, handle_start, path_id, paths_number)
            } else {
                //ricava handle id pos prima, ricava suo handle end e aggiorna hash
                let succ = path_nodes[pos - 1];
                let succ_end = handles_id_position.get(&succ.id()).unwrap().1;
                succ_hash_struct.set_succs_and_paths(
                    succ_end as usize,
                    handle_start,
                    path_id,
                    paths_number,
                );

                // se ultimo nodo path aggiorna anche F
                if pos == path_nodes.iter().len() - 1 {
                    succ_hash_struct.set_succs_and_paths(
                        handle_end,
                        linearization.len() - 1,
                        path_id,
                        paths_number,
                    );
                }
            }
        }
    }
    nodes_with_succ.set(0, true);
    paths_nodes[linearization.len() - 1] = BitVec::from_elem(paths_number, true);

    PathGraph::build(
        linearization,
        nodes_with_succ,
        succ_hash_struct,
        paths_nodes,
        paths_number,
        nodes_id_pos,
    )
}

/// Returns a vector of (read, read_name) from a .fasta file, ready for the alignment
pub fn read_sequence_w_path(file_path: &str) -> Vec<char> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut sequence: Vec<char> = Vec::new();
    for line in reader.lines().flatten() {
        if !line.starts_with('>') && !line.is_empty() {
            let mut line: Vec<char> = line
                .chars()
                .map(|c| {
                    if c == '-' {
                        'N'
                    } else {
                        c.to_ascii_uppercase()
                    }
                })
                .collect::<Vec<char>>();
            sequence.append(&mut line);
        }
    }
    if !sequence.is_empty() {
        sequence.insert(0, '$');
    }
    sequence //update with also sequences_name
}

pub fn output_formatter(recombs: &Vec<((&usize, BitVec), (&usize, BitVec))>, graph: &PathGraph) {
    for ((i, i_paths), (j, j_paths)) in recombs {
        let i_node = graph.nodes_id_pos[**i];
        let i_offset = get_offset(**i, i_node, graph);

        let j_node = graph.nodes_id_pos[**j];
        let j_offset = get_offset(**j, j_node, graph);

        println!("{i_node}[{i_offset}]\t{j_node}[{j_offset}]")

    }
}

fn get_offset(i: usize, node: u64, graph: &PathGraph) -> u64{
    let mut offset = 0;
    let mut start = i;
    while graph.nodes_id_pos[start - 1] == node {
        start -= 1;
        offset += 1;
    }
    offset
}