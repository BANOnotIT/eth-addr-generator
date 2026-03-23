use alloy_signer_local::PrivateKeySigner;
use clap::Parser;
use coins_bip39::{English, Mnemonic};
use rand::seq::SliceRandom;
use rand::{Rng, RngExt};
use rand_chacha::ChaCha20Rng;
use rayon::prelude::*;

#[derive(Parser)]
#[command(about = "Generate Ethereum address/key pairs")]
struct Args {
    #[arg(short, long, default_value_t = 10)]
    count: usize,

    #[arg(short, long)]
    phrase: bool,
}

fn main() {
    let args = Args::parse();
    let n = args.count;

    let num_threads = rayon::current_num_threads();
    let per_thread = (n + num_threads - 1) / num_threads;

    let mut results: Vec<String> = (0..num_threads)
        .into_par_iter()
        .flat_map(|_| {
            if args.phrase {
                generate_phrases(per_thread)
            } else {
                generate_private_keys(per_thread)
            }
        })
        .collect();

    results.shuffle(&mut rand::rng());

    for item in results.iter().take(n) {
        println!("{}\n", item);
    }
}

fn generate_private_keys(n: usize) -> Vec<String> {
    let mut rng: ChaCha20Rng = rand::make_rng();
    let mut pairs = Vec::with_capacity(n);
    for _ in 0..n {
        let mut bytes = [0u8; 32];
        rng.fill_bytes(&mut bytes);
        let signer = PrivateKeySigner::from_slice(&bytes).unwrap();
        pairs.push(format!("{}\n\t{:x}", signer.address(), signer.to_bytes()));
    }
    pairs
}

fn generate_phrases(n: usize) -> Vec<String> {
    let mut rng: ChaCha20Rng = rand::make_rng();
    let mut pairs = Vec::with_capacity(n);
    for _ in 0..n {
        let mut bytes = [0u8; 16];
        rng.fill(&mut bytes);

        let mnemonic = Mnemonic::<English>::new_from_entropy(bytes.into());
        pairs.push(mnemonic.to_phrase());
    }
    pairs
}
