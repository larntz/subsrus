extern crate subsrus;
extern crate bytes;

use subsrus::hasher;
use subsrus::subsdb;
use std::fs::File;
use std::io::prelude::*;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect(); 

    match args.get(1) {
        Some(a) => {
            let hash = hasher::file_hash(a);
            println!("hash: {}", &hash);

            let mut f = File::create("subtitle.srt").expect("file create subtitle.srt failed");
            f.write_all(&subsdb::get_subtitle(&hash).expect("binary crash: beep boop"))
                .expect("file write subtitle.srt failed");
        },
        None    => println!("None"),
    }
}
