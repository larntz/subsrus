use reqwest;
use bytes::Bytes;
use std::env;

pub fn search_available(hash: &str) -> Result<String, reqwest::Error>
{
    match env::var("SANDBOX")
    {
        Ok(_) => {
            let url = format!("http://sandbox.thesubdb.com/?action=search&hash={}", hash);
            println!("sandbox url: {}", &url);
            let response = send_get_request(&url).expect("request_subtitle (sandbox) failed");
            println!("headers: {:?}", response.headers());
            response.text()
        },
        Err(_) => {
            let url = format!("http://api.thesubdb.com/?action=search&hash={}", hash);
            let response = send_get_request(&url).expect("request_subtitle (api) failed");
            response.text()
        }
    }
    
}

pub fn post_subtitle(hash: &str, subtitle: Vec<u8>) -> String
{
    let form = reqwest::blocking::multipart::Form::new()
        .text("hash", hash.to_string())
        .part(
            "file",
            reqwest::blocking::multipart::Part::bytes(subtitle)
            .file_name("subtitle.srt")
            .mime_str("application/octet-stream").unwrap()
            );

    let mut url = "http://api.thesubdb.com/?action=upload";
    if env::var("SANDBOX").is_err()
    {
        url = "http://sandbox.thesubdb.com/?action=upload";
    }

    match send_post_request(url, form).unwrap().status().as_u16()
    {
        201 => String::from("Sutitle successfully uploaded."),
        403 => String::from("Subtitle already in database."),
        415 => String::from("Subtitle format unsupported."),
        400 => String::from("Bad Request"),
        _ => String::from("Unknown Response")
    }
}

fn send_post_request(url: &str, form: reqwest::blocking::multipart::Form) -> Result<reqwest::blocking::Response, reqwest::Error>
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

    println!("agent: {:?}", user_agent);
    println!("form: {:?}", form);
    let client = reqwest::blocking::Client::builder()
        .user_agent(user_agent)
        .build()?
        .post(url)
        .multipart(form);

    println!("client: {:?}", client);

    client.send()
}

pub fn get_subtitle(hash: &str, lang: &str) -> Result<Bytes, reqwest::Error>
{
    match env::var("SANDBOX")
    {
        Ok(_) => {
            let url = format!("http://sandbox.thesubdb.com/?action=download&hash={}&language={}", hash, lang);
            println!("sandbox url: {}", &url);
            let response = send_get_request(&url).expect("request_subtitle (sandbox) failed");
            println!("headers: {:?}", response.headers());
            response.bytes()
        },
        Err(_) => {
            let url = format!("http://api.thesubdb.com/?action=download&hash={}&language={}", hash, lang);
            let response = send_get_request(&url).expect("request_subtitle (api) failed");
            response.bytes()
        }
    }
}

fn send_get_request(url: &str) -> Result<reqwest::blocking::Response, reqwest::Error>
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
    fn test_subsdb_post_subtitle()
    {
        use std::io::prelude::*;
        use std::fs::File;
        std::env::set_var("SANDBOX","test");
        let mut f = File::open("test_vids/justified.en.srt")
            .expect("failed to open srt file");
        
        let mut buf = Vec::new();
        f.read_to_end(&mut buf)
            .expect("failed to read subtitle file");

        assert_eq!(subsdb::post_subtitle("edc1981d6459c6111fe36205b4aff6c2", buf),
            "Subtitle already in database.");
    }

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
