use std::vec;

use crate::graph_parser::PathGraph;

pub fn extract_kmers(graph: &PathGraph, k: usize) {
    let graph_len = graph.lnz.len();
    let starting_positions = graph.succ_hash.get_succs_and_paths(0);

    for (path_start, _) in starting_positions {
        let mut window_pos = path_start;
        
        let mut kmer = vec![graph.lnz[window_pos]];

        while window_pos < graph_len - 1 {
            let mut idx = window_pos;
            while kmer.len() < k {
                println!("{idx}");
               
                if !graph.nws[idx] {
                    idx += 1;
                    kmer.push(graph.lnz[idx]);
                } else {
                    println!("ELSE");
                    for (succ_idx, _) in graph.succ_hash.get_succs_and_paths(window_pos + idx) {
                        recursive_extraction(graph, &mut kmer, k, succ_idx);
                    }
                   
                }
            }
            println!("MAIN {:?}", kmer);
            window_pos += 1;
    
            }
        }
    }

    fn recursive_extraction(graph: &PathGraph, kmer: &mut Vec<char>, k: usize, mut idx: usize) {
        kmer.push(graph.lnz[idx]);
        while kmer.len() < k {
            if !graph.nws[idx] {
                idx += 1;
                kmer.push(graph.lnz[idx]);
            } else {
                for (succ_idx, _) in graph.succ_hash.get_succs_and_paths(idx) {
                    recursive_extraction(graph, kmer, k, succ_idx);
                }
               
            }
        }
        println!("RECU {:?}", kmer)
    }