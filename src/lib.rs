pub mod hasher;
pub mod subsdb;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use bytes::Bytes;

pub fn download(source: &str, langs: &str)
{
    println!("\nSearch for subtitles:\n\t{}\n", source);

    let hash = hasher::file_hash(source);
    let available_langs = subsdb::search_available(&hash).expect("search_available failed");

    let _: Vec<_> = langs.split(',').map(|lang| {
        if available_langs.contains(lang)
        {
            write_srt_file(
                get_srt_filename(source, lang),
                &subsdb::get_subtitle(&hash,&lang).expect("failed to download subtitle")
                );
        }
        else
        {
            println!("Subitle not avilable for language='{}'\n", lang);
        }
    }).collect();
}

fn write_srt_file(filename: String, contents: &Bytes)
{
    println!("Writing srt file: \n{}\n", filename);

    let mut f = File::create(filename)
        .expect("file create subtitle.srt failed");
    f.write_all(contents).expect("writing output file failed");
}

fn get_srt_filename(source: &str, lang: &str) -> String
{
    let path = Path::new(source);
    let mut srt_file = path.to_path_buf();
    srt_file.set_extension(format!("{}.srt", lang));
    srt_file.to_str().expect("filename to_str failed").to_string()
}


#[cfg(test)]
mod tests {
    use std::path::Path;

    #[test]
    fn test_path()
    {
        let path = Path::new("test_vids/justified.mp4");

        println!("path: {}", path.display() );

        let mut pbuf = path.to_path_buf();
        pbuf.set_extension("es.srt");

        println!("newpath: {:?}", pbuf);
            
        assert_eq!(pbuf.to_str().unwrap(), "test_vids/justified.es.srt");

    }

}
