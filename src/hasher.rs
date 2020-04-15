extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::fs::File;

const READSIZE: usize = 64 * 1024;

pub fn file_hash(name: &str) -> String
{
   let mut x =  Md5::new();
   x.input(&file_bytes(&name));
   String::from(x.result_str())
}

fn file_bytes(name: &str) -> [u8; READSIZE * 2 ]
{
    let mut f = File::open(name).expect("oops::open");
    let mut buffer = [0; READSIZE * 2 ];
    f.read(&mut buffer[..READSIZE]).expect("oops::start");
    f.seek(SeekFrom::End(-1 * READSIZE as i64)).expect("oops::seek");
    f.read(&mut buffer[READSIZE..(READSIZE * 2)]).expect("oops::end");
    buffer
}
