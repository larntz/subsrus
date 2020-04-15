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
            println!("headers: {:?}", response.headers());
            response.bytes()
        },
        Err(_) => {
            let url = format!("http://api.thesubdb.com/?action=download&hash={}&language=es,en", hash);
            let response = request_subtitle(&url).expect("request_subtitle (api) failed");
            println!("headers: {:?}", response.headers());
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

    if env::var("SANDBOX").is_ok()
    {
        println!("user_agent: {}", &user_agent);
    }

    reqwest::blocking::Client::builder()
        .user_agent(user_agent)
        .build()?
        .get(url)
        .send()
}


#[cfg(test)]
mod tests {
    use crate::subsdb;    

    #[test]
    fn test_subsdb_get_subtitle()
    {
        std::env::set_var("SANDBOX","yes");
        let subtitle = subsdb::get_subtitle("edc1981d6459c6111fe36205b4aff6c2").expect("Something went wrong");
        println!("\n\n{:?}\n\n", subtitle);    
    }
}
