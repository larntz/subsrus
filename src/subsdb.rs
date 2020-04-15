extern crate reqwest;
extern crate bytes;

use std::env;

pub fn get_subtitle(hash: &str) -> Result<bytes::Bytes, reqwest::Error>
{
    match env::var("SANDBOX")
    {
        Ok(_) => {
            let url = format!("http://sandbox.thesubdb.com/?action=download&hash={}&language=es,en", hash);
            println!("sandbox url: {}", &url);
            let response = request_subtitle(&url).expect("request_subtitle (sandbox) failed");
            response.bytes()
        },
        Err(_) => {
            let url = format!("http://api.thesubdb.com/?action=download&hash={}&language=es,en", hash);
            let response = request_subtitle(&url).expect("request_subtitle (api) failed");
            response.bytes()
        }
    }

}

fn request_subtitle(url: &str) -> Result<reqwest::blocking::Response, reqwest::Error>
{
    let user_agent = format!("SubDB/1.0 ({}/{}; {})", 
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_HOMEPAGE")
        );

    println!("user_agent: {}", &user_agent);

    let client = reqwest::blocking::Client::builder()
        .user_agent(user_agent)
        .build()?;

    client.get(url).send()
}
