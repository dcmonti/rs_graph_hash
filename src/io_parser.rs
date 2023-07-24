use bio;
use bit_vec::BitVec;
use gfa::{gfa::*, parser::GFAParser};
use handlegraph::{
    handle::{Handle, NodeId},
    handlegraph::HandleGraph,
    hashgraph::HashGraph,
};
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter, Write},
    path::Path,
};

use crate::{
    cli,
    path_graph::{PathGraph, SuccHash},
    rec_struct::RecStruct,
};

pub fn read_graph_w_path(file_path: &str) -> PathGraph {
    let parser = GFAParser::new();
    let gfa: GFA<usize, ()> = parser.parse_file(file_path).unwrap();

    let graph: HashGraph = HashGraph::from_gfa(&gfa);
    create_path_graph(&graph)
}

pub fn create_path_graph(graph: &HashGraph) -> PathGraph {
    let mut sorted_handles = graph.handles_iter().collect::<Vec<Handle>>();
    sorted_handles.sort();

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
        let path_nodes = path.nodes.iter().collect::<Vec<&Handle>>();
        for (pos, handle) in path_nodes.iter().enumerate() {
            let (handle_start, handle_end) = handles_id_position.get(&handle.id()).unwrap();
            let handle_start = *handle_start as usize;
            let handle_end = *handle_end as usize;

            for idx in handle_start..=handle_end {
                paths_nodes[idx].set(path_id, true);
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
pub fn read_sequence_w_path(file_path: &str, amb_mode: bool) -> Vec<(String, String)> {
    let file = File::open(file_path).unwrap();
    let buffer = BufReader::new(file);
    let reader = bio::io::fasta::Reader::new(buffer);

    let mut sequences = Vec::new();
    for result in reader.records() {
        let record = result.expect("Error parsing FASTA file");
        let mut b_read = record.seq().to_owned().to_ascii_uppercase();

        let read_id = record.id().to_owned();

        let read: String = String::from_utf8(b_read.clone()).unwrap();

        let mut pos_read_id = read_id.clone();
        pos_read_id.push('+');
        sequences.push((pos_read_id, read));
        if amb_mode {
            b_read.reverse();
            let mut rev_and_compl = Vec::new();
            for c in b_read.iter() {
                rev_and_compl.push(bio::alphabets::dna::complement(*c));
            }
            let mut rev_read_id = read_id.clone();
            rev_read_id.push('-');
            sequences.push((rev_read_id, String::from_utf8(rev_and_compl).unwrap()))
        }
    }
    sequences
}

pub fn output_formatter(recombs: &Vec<RecStruct>, graph: &PathGraph, id: &String) {
    let mut outputs = String::new();
    let out_path: String = cli::get_out_file();

    for rec in recombs {
        let i = rec.first_start;
        let ie = rec.first_end;
        let i_paths = &rec.first_paths;
        let j = rec.second_start;
        let je = rec.second_end;
        let j_paths = &rec.second_paths;

        let i_node_start = graph.nodes_id_pos[i];
        let i_node_end = graph.nodes_id_pos[ie];
        let i_offset_start = get_offset(i, i_node_start, graph);
        let i_offset_end = get_offset(ie, i_node_end, graph);
        let i_paths_id = get_paths(i_paths);

        let j_node_start = graph.nodes_id_pos[j];
        let j_node_end = graph.nodes_id_pos[je];
        let j_offset_start = get_offset(j, j_node_start, graph);
        let j_offset_end = get_offset(je, j_node_end, graph);
        let j_paths_id = get_paths(j_paths);
        let output = format!(
            "{}\t{}[{}]\t{}[{}]\t{}\t{}\t{}[{}]\t{}[{}]\t{}\t{}",
            id,
            i_node_start,
            i_offset_start,
            i_node_end,
            i_offset_end,
            i_paths_id,
            rec.first_read_start,
            j_node_start,
            j_offset_start,
            j_node_end,
            j_offset_end,
            j_paths_id,
            rec.second_read_start
        );
        outputs = format!("{}\n{}", outputs, output)
    }
    outputs = outputs.trim().to_string();

    if outputs.is_empty() {
        outputs = format!("{id}\tno rec")
    }
    if out_path == "standard output" {
        println!("{outputs}");
    } else {
        write_output(&outputs)
    }
}

pub fn write_output(recombs: &str) {
    let mut out_file = cli::get_out_file();
    out_file.push_str(".rec");

    let file_name = Path::new(&out_file);
    let file = if file_name.exists() {
        OpenOptions::new()
            .write(true)
            .append(true)
            .open(file_name)
            .unwrap()
    } else {
        File::create(file_name).expect("unable to create file")
    };

    let mut f = BufWriter::new(&file);
    writeln!(f, "{}", recombs.trim()).expect("error in writing");
}

fn get_offset(i: usize, node: u64, graph: &PathGraph) -> u64 {
    let mut offset = 0;
    let mut start = i;
    while graph.nodes_id_pos[start - 1] == node {
        start -= 1;
        offset += 1;
    }
    offset
}

fn get_paths(v: &BitVec) -> String {
    let mut paths_vec = Vec::new();
    for (path, is_present) in v.iter().enumerate() {
        if is_present {
            paths_vec.push((path + 1).to_string())
        }
    }
    paths_vec.join(",")
}
