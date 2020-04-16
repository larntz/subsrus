#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let langs = matches.value_of("languages").unwrap_or("en");
    let source = matches.value_of("source_file").expect("source file error");

    subsrus::download(source, langs);
}
