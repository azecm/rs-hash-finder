mod find_hash;
mod types;

use clap::Parser;
use crate::find_hash::find_all_hash;
use crate::types::Args;


fn main() {
    let params = Args::parse();
    let cpus = num_cpus::get();
    let results = find_all_hash(cpus, params);
    for (val, hash) in results.iter() {
        println!("{val}, {hash}");
    }
}

