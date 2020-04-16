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

#[cfg(test)]
mod tests {
    use crate::hasher;

    const JUSTIFIED: &str = "./test_vids/justified.mp4";
    const DEXTER: &str = "./test_vids/dexter.mp4";

    #[test]
    fn test_hasher_mp4_file_hash()
    {
        assert_eq!(hasher::file_hash(JUSTIFIED), "edc1981d6459c6111fe36205b4aff6c2");
        assert_eq!(hasher::file_hash(DEXTER),"ffd8d4aa68033dc03d1c8ef373b9028c");
    }
}
