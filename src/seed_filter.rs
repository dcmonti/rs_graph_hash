
use crate::seed_kmer::SeedKmer;

/*
pub fn filter_old(seeds: &mut Vec<SeedKmer>, k_len: usize) -> Vec<SeedKmer> {
    seeds.retain(|seed| {
        let read_start = seed.positions[2].offset;
        let read_end = seed.positions[3].offset;
        read_end - read_start > k_len * 2
    });
    
    let mut new_seeds = Vec::new();
    new_seeds.push(seeds[0].clone());
    for seed in seeds.iter().skip(1) {
        let last_seed = new_seeds.last_mut().unwrap();
        let mut common_paths = seed.paths.clone();
        common_paths.and(&last_seed.paths);
        if common_paths.any() {
            last_seed.update_ends(seed.positions[1], seed.positions[3], common_paths);
        } else {
            new_seeds.push(seed.to_owned())
        }
    }
    new_seeds
}
*/

pub fn filter(seeds: &mut Vec<SeedKmer>, k_len: usize) {
    // rimuovi seed corti
    seeds.retain(|seed| seed.positions[3].offset - seed.positions[2].offset > k_len * 2);

    let mut idx = 1;

    //unisci seed consecutivi su stesso path dopo rimozione intermedi
    while idx < seeds.len() {
        let (prev_seed, rest) = seeds.split_at_mut(idx);
        let prev_seed = &mut prev_seed[idx - 1];

        if let Some(seed) = rest.first_mut() {
            let mut common_paths = seed.paths.clone();
            common_paths.and(&prev_seed.paths.clone());            
            if common_paths.any() {
                prev_seed.update_ends(seed.positions[1], seed.positions[3], common_paths);
                seeds.remove(idx);
            } else {
                idx += 1;
            }
        } else {
            break;
        }
    }
}
