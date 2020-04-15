extern crate subsrus;
use subsrus::hasher;
use subsrus::subsdb;

use std::env;

fn main() {
    let args: Vec<_> = env::args().collect(); 

    match args.get(1) {
        Some(a) => {
            let hash = hasher::file_hash(a);
            println!("hash: {}", &hash);

            println!("\n\n{}\n\n", subsdb::get_subtitle(&hash).expect("binary crash: beep boop"));
        },
        None    => println!("None"),
    }
}
