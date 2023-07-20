use std::{
    collections::{HashMap, VecDeque},
    vec,
};

use bit_vec::BitVec;

use crate::graph_parser::PathGraph;

pub fn extract_unique_kmers(graph: &PathGraph, k: usize) {
    let mut unique_kmers = HashMap::new();
    let mut found_kmers = HashMap::new();

    let starting_positions = graph.succ_hash.get_succs_and_paths(0);
    let mut positions = VecDeque::new();
    let mut visited_nodes = BitVec::from_elem(
        *graph.nodes_id_pos.iter().max().unwrap() as usize + 1,
        false,
    );
    for (path_start, _) in starting_positions {
        if !positions.contains(&path_start) {
            positions.push_front(path_start);
        }
    }

    while !positions.is_empty() {
        let window_pos = positions.pop_front().unwrap();
        if !visited_nodes[graph.nodes_id_pos[window_pos] as usize] {
            let mut kmer = vec![];
            let mut paths = BitVec::from_elem(graph.paths_number, true);
            recursive_extraction(
                graph,
                &mut kmer,
                &mut paths,
                k,
                window_pos,
                &mut found_kmers,
                &mut unique_kmers,
                window_pos,
            );

            if graph.nws[window_pos] {
                visited_nodes.set(graph.nodes_id_pos[window_pos] as usize, true);
                for (succ, _) in graph.succ_hash.get_succs_and_paths(window_pos) {
                    if graph.lnz[succ] != 'F' {
                        positions.push_front(succ);
                    }
                }
            } else {
                positions.push_front(window_pos + 1)
            }
        }
    }
    for (k, v) in unique_kmers.iter() {
        println!("{:?}\t{:?}", k, v);
    }
}

fn recursive_extraction(
    graph: &PathGraph,
    kmer: &Vec<char>,
    paths: &mut BitVec,
    k: usize,
    idx: usize,
    found_kmers: &mut HashMap<Vec<char>, (usize, BitVec)>,
    unique_kmers: &mut HashMap<Vec<char>, (usize, BitVec)>,
    kmer_start: usize,
) {
    let mut loc_kmer = kmer.clone();
    let mut loc_paths = paths.clone();

    loc_kmer.push(graph.lnz[idx]);
    loc_paths.and(&graph.paths_nodes[idx]);

    if loc_kmer.len() < k && idx < graph.lnz.len() - 1 {
        if !graph.nws[idx] {
            recursive_extraction(
                graph,
                &mut loc_kmer,
                &mut loc_paths,
                k,
                idx + 1,
                found_kmers,
                unique_kmers,
                kmer_start,
            );
        } else {
            for (succ_idx, _) in graph.succ_hash.get_succs_and_paths(idx) {
                if graph.lnz[succ_idx] != 'F' {
                    recursive_extraction(
                        graph,
                        &mut loc_kmer,
                        &mut loc_paths,
                        k,
                        succ_idx,
                        found_kmers,
                        unique_kmers,
                        kmer_start,
                    );
                } else {
                    return;
                }
            }
        }
    }
    if loc_kmer.len() == k && loc_paths.any() {
        if found_kmers.contains_key(&loc_kmer) {
            //TODO: determinare esattamente cosa sono i kmer unici
            unique_kmers.remove(&loc_kmer);
        } else {
            found_kmers.insert(loc_kmer.clone(), (kmer_start, loc_paths.clone()));
            unique_kmers.insert(loc_kmer.clone(), (kmer_start, loc_paths.clone()));
        }

        //println!("K-MER: {:?}\tPATHS: {:?}", loc_kmer.clone(), loc_paths);
    }
}
