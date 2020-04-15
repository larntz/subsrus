

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    #[test]
    fn test_path()
    {
        let path = Path::new("/mnt/media/Movies/The Notebook (2004) [1080p]/The.Notebook.2004.1080p.BluRay.x264.YIFY.mp4");

        println!("path: {}", path.display() );
        //println!("parent: {:?}", path.parent());
        //println!("file_stem: {:?}", path.file_stem());
        //println!("extension: {:?}", path.extension());


        let mut pbuf = path.to_path_buf();
        pbuf.set_extension("es.srt");

        println!("newpath: {:?}", pbuf);
            
        assert!(false);

    }


}
