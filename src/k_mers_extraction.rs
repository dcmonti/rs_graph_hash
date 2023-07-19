use std::vec;

use crate::graph_parser::PathGraph;

pub fn extract_kmers(graph: &PathGraph, k: usize) {
    //POSITIONS DEVE ESSERE UNA CODA
    let graph_len = graph.lnz.len();
    let starting_positions = graph.succ_hash.get_succs_and_paths(0);
    let mut positions = vec![];
    for (path_start, _) in starting_positions {
        positions.push(path_start);
    }
        
    for window_pos in &mut positions {
            let mut kmer = vec![];
            recursive_extraction(graph, &mut kmer, k, *window_pos);
            println!("{:?}", window_pos);

            if graph.nws[*window_pos] {

            } else {
                positions.push(*window_pos + 1)
            }

    
    }

    }

    fn recursive_extraction(graph: &PathGraph, kmer: &mut Vec<char>, k: usize, mut idx: usize) {
        
        kmer.push(graph.lnz[idx]);
        while kmer.len() < k && idx < graph.lnz.len() - 1 {
            
            if !graph.nws[idx] {
                idx += 1;
                kmer.push(graph.lnz[idx]);
            } else {
                for (succ_idx, _) in graph.succ_hash.get_succs_and_paths(idx) {
                    if graph.lnz[succ_idx] != 'F' {
                        recursive_extraction(graph, kmer, k, succ_idx);
                    } else {
                        return;
                    }
                }
               
            }
        }
        println!("{:?}", kmer)
    }