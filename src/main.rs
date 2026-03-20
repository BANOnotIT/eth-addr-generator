use alloy_signer_local::PrivateKeySigner;
use clap::Parser;
use rand::Rng;
use rand::seq::SliceRandom;
use rand_chacha::ChaCha20Rng;
use rayon::prelude::*;

#[derive(Parser)]
#[command(about = "Generate Ethereum address/key pairs")]
struct Args {
    #[arg(short, long, default_value_t = 10)]
    count: usize,
}

fn main() {
    let args = Args::parse();
    let n = args.count;

    let num_threads = rayon::current_num_threads();
    let per_thread = (n + num_threads - 1) / num_threads;

    let mut results: Vec<_> = (0..num_threads)
        .into_par_iter()
        .flat_map(|_| {
            let mut rng: ChaCha20Rng = rand::make_rng();
            let mut pairs = Vec::with_capacity(per_thread);
            for _ in 0..per_thread {
                let mut bytes = [0u8; 32];
                rng.fill_bytes(&mut bytes);
                let signer = PrivateKeySigner::from_slice(&bytes).unwrap();
                pairs.push((signer.address(), signer.to_bytes()));
            }
            pairs
        })
        .collect();

    results.shuffle(&mut rand::rng());

    for (addr, key) in results.iter().take(n) {
        println!("{addr}");
        println!("\t0x{key:x}");
    }
}
