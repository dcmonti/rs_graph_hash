use bio;
use bit_vec::BitVec;
use gfa::{gfa::*, parser::GFAParser};
use handlegraph::hashgraph::HashGraph;
use std::{
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter, Write},
    path::Path,
};

use crate::{cli, seed_kmer::SeedKmer};

pub fn read_graph_w_path(file_path: &str) -> HashGraph {
    let parser = GFAParser::new();
    let gfa: GFA<usize, ()> = parser.parse_file(file_path).unwrap();

    let graph: HashGraph = HashGraph::from_gfa(&gfa);
    graph
}

/// Returns a vector of (read, read_name) from a .fasta file
pub fn read_sequence_w_path(file_path: &str, amb_mode: bool) -> Vec<(String, String)> {
    let file = File::open(file_path).unwrap();
    let buffer = BufReader::new(file);
    let reader = bio::io::fasta::Reader::new(buffer);

    let mut sequences = Vec::new();
    for result in reader.records() {
        let record = result.expect("Error parsing FASTA file");
        let mut b_read = record.seq().to_owned().to_ascii_uppercase();

        let read_id = record.id().to_owned();

        let read: String = String::from_utf8_lossy(&b_read).to_string();

        let mut pos_read_id = read_id.clone();
        pos_read_id.push('+');
        sequences.push((pos_read_id, read));
        if amb_mode {
            b_read.reverse();
            let b_read: Vec<u8> = b_read
                .iter()
                .map(|c| bio::alphabets::dna::complement(*c))
                .collect();

            let mut rev_read_id = read_id.clone();
            rev_read_id.push('-');
            sequences.push((rev_read_id, String::from_utf8(b_read).unwrap()))
        }
    }
    sequences
}

pub fn output_formatter(seeds: &Vec<SeedKmer>, id: &String) {
    let mut outputs = String::new();
    let out_path: String = cli::get_out_file();

    for seed in seeds {
        let kmer_start = seed.positions[0];
        let kmer_end = seed.positions[1];
        let read_start = seed.positions[2].offset;
        let read_end = seed.positions[3].offset;

        let start_node_id = kmer_start.node_id;
        let end_node_id = kmer_end.node_id;
        let start_offset = kmer_start.offset;
        let end_offset = kmer_end.offset;

        let paths = &seed.paths;
        let paths_id = get_paths(paths);

        let output = format!(
            "{}\t{}[{}]\t{}[{}]\t{}\t{}\t{}",
            id,
            start_node_id,
            start_offset,
            end_node_id,
            end_offset,
            paths_id,
            read_start,
            read_end
        );
        outputs = format!("{}\n{}", outputs, output)
    }
    outputs = outputs.trim().to_string();

    if outputs.is_empty() {
        outputs = format!("{id}\tno match")
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

fn get_paths(v: &BitVec) -> String {
    let mut paths_vec = Vec::new();
    for (path, is_present) in v.iter().enumerate() {
        if is_present {
            paths_vec.push((path).to_string())
        }
    }
    paths_vec.join(",")
}
