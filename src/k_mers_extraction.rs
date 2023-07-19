use std::{vec, collections::VecDeque};

use bit_vec::BitVec;

use crate::graph_parser::PathGraph;


pub fn extract_kmers(graph: &PathGraph, k: usize) {
    let starting_positions = graph.succ_hash.get_succs_and_paths(0);
    let mut positions = VecDeque::new();
    let mut visited_nodes = BitVec::from_elem(*graph.nodes_id_pos.iter().max().unwrap() as usize + 1, false);
    for (path_start, _) in starting_positions {
        if !positions.contains(&path_start) {
            positions.push_front(path_start);
            
        }
    }
        
    while !positions.is_empty() {
        let window_pos = positions.pop_front().unwrap();
        if !visited_nodes[graph.nodes_id_pos[window_pos] as usize]{
            
            let mut kmer = vec![];
            recursive_extraction(graph, &mut kmer, k, window_pos);

            if graph.nws[window_pos] {
                visited_nodes.set(graph.nodes_id_pos[window_pos] as usize, true);
                for (succ, _) in graph.succ_hash.get_succs_and_paths(window_pos){
                    if graph.lnz[succ] != 'F' {
                        positions.push_front(succ);
                    }
                }
            } else {
                positions.push_front(window_pos+1)
            }
        }
        

    
    }

    }

    fn recursive_extraction(graph: &PathGraph, kmer: &Vec<char>, k: usize, idx: usize) {
        let mut loc_kmer = kmer.clone();
        loc_kmer.push(graph.lnz[idx]);
        if loc_kmer.len() < k && idx < graph.lnz.len() - 1 {
            
            if !graph.nws[idx] {
                recursive_extraction(graph, &mut loc_kmer, k, idx+1);

            } else {
                for (succ_idx, _) in graph.succ_hash.get_succs_and_paths(idx) {
                    if graph.lnz[succ_idx] != 'F' {

                        recursive_extraction(graph, &mut loc_kmer, k, succ_idx);
                    } else {
                        return;
                    }
                }
               
            }
        }
        if loc_kmer.len() == k {
            println!("{:?}", loc_kmer);

        }

        //println!("{:?}", idx);
        
    }