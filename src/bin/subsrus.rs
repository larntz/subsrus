#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    
    match matches.subcommand()
    {
        ("download", Some(download)) =>
        {
            subsrus::download(
                download.value_of("source_video").expect("source video error"),
                download.value_of("languages").unwrap_or("en")
            );
        },
        ("upload", Some(upload)) =>
        {
            subsrus::upload(
                upload.value_of("source_video").expect("source video error"),
                upload.value_of("source_subtitle").expect("source subtitle error"),
            );
        },
        _ => println!("What do you want from me?")
    }
}
