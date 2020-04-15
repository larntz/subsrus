pub mod hasher;
pub mod subsdb;

#[cfg(test)]
mod tests {
    use super::hasher;
    use super::subsdb;
    use std::env;

    const JUSTIFIED: &str = "./test_vids/justified.mp4";
    const DEXTER: &str = "./test_vids/dexter.mp4";

    #[test]
    fn test_hasher_mp4_file_hash()
    {
        assert_eq!(hasher::file_hash(JUSTIFIED), "edc1981d6459c6111fe36205b4aff6c2");
        assert_eq!(hasher::file_hash(DEXTER),"ffd8d4aa68033dc03d1c8ef373b9028c");
    }

    #[test]
    fn test_subsdb_get_subtitle()
    {
        env::set_var("SANDBOX","yes");
        let subtitle = subsdb::get_subtitle(&hasher::file_hash(JUSTIFIED)).expect("Something went wrong");
        println!("\n\n{:?}\n\n", subtitle);    
    }

}

