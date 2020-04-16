extern crate reqwest;
extern crate bytes;

use std::env;

pub fn search_available(hash: &str) -> Result<String, reqwest::Error>
{
    match env::var("SANDBOX")
    {
        Ok(_) => {
            let url = format!("http://sandbox.thesubdb.com/?action=search&hash={}", hash);
            println!("sandbox url: {}", &url);
            let response = send_request(&url).expect("request_subtitle (sandbox) failed");
            println!("headers: {:?}", response.headers());
            response.text()
        },
        Err(_) => {
            let url = format!("http://api.thesubdb.com/?action=search&hash={}", hash);
            let response = send_request(&url).expect("request_subtitle (api) failed");
            response.text()
        }
    }
    
}

pub fn get_subtitle(hash: &str, lang: &str) -> Result<bytes::Bytes, reqwest::Error>
{
    match env::var("SANDBOX")
    {
        Ok(_) => {
            let url = format!("http://sandbox.thesubdb.com/?action=download&hash={}&language={}", hash, lang);
            println!("sandbox url: {}", &url);
            let response = send_request(&url).expect("request_subtitle (sandbox) failed");
            println!("headers: {:?}", response.headers());
            response.bytes()
        },
        Err(_) => {
            let url = format!("http://api.thesubdb.com/?action=download&hash={}&language={}", hash, lang);
            let response = send_request(&url).expect("request_subtitle (api) failed");
            response.bytes()
        }
    }
}

fn send_request(url: &str) -> Result<reqwest::blocking::Response, reqwest::Error>
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
        std::env::set_var("SANDBOX","test");
        let subtitle = subsdb::get_subtitle("edc1981d6459c6111fe36205b4aff6c2","en").expect("Something went wrong");
        println!("\n\n{:?}\n\n", subtitle);    
    }
    #[test]
    fn test_subsdb_search_available()
    {
        std::env::set_var("SANDBOX","yes");
        let available = subsdb::search_available("edc1981d6459c6111fe36205b4aff6c2").expect("Something went wrong");
        println!("\n\n{:?}\n\n", available);    
        assert_eq!("en,es,fr,it,pt", available);
    }
}
