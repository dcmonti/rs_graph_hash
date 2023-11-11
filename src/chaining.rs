use handlegraph::hashgraph::HashGraph;

use crate::{seed_kmer::SeedKmer, extract_subgraph};

pub fn execute(
    seeds: &Vec<SeedKmer>,
    graph: &HashGraph,
    read: &String,
) {
    if seeds.len() >= 2 {
        for i in 0..seeds.len() - 1 {
            let this_seed = seeds.get(i).unwrap();
            let next_seed = seeds.get(i + 1).unwrap();
            // se seeds a distanza uno
            // merge_seeds(&this_seed, &next_seed)
            let sub = extract_subgraph::execute(
                &graph,
                this_seed.positions[1].node_id,
                this_seed.positions[1].offset,
                next_seed.positions[0].node_id,
                next_seed.positions[0].offset,
            );
            let subread = read
                [this_seed.positions[3].offset..next_seed.positions[2].offset]
                .to_string();
            let mut res = recgraph::api::align_local_no_gap(&subread, &sub, None, None);
            res.query_start += this_seed.positions[3].offset;
            res.query_end += this_seed.positions[3].offset;
            println!("{}", res.to_string());
            // merge seeds and gaf_struct
        }
    }
}