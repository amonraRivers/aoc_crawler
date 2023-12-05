use reqwest::{self, header::HeaderValue};
use std::fs;
use std::io::{BufWriter, Read, Write};
fn main() {
    let year = "2023";
    let day = "2";
    let client = reqwest::blocking::Client::new();
    let base_url = String::from("https://adventofcode.com/") + year + "/day/";
    let origin_url = base_url + day + "/input";
    let mut headers = reqwest::header::HeaderMap::new();
    let cookie="_ga=GA1.2.1179491718.1700792032; _gid=GA1.2.927193133.1701390504; session=53616c7465645f5f079a808e82be0c643a8d77d734f65cb06e90051c570ee785388cda7bb9462b4d79ef3e990854b7f31ef9c6e43cc942ba71bfe505bd301b50;";
    let hv = HeaderValue::from_str(cookie).unwrap();
    headers.append(reqwest::header::COOKIE, hv);

    let mut res = client.get(&origin_url).headers(headers).send().unwrap();
    println!("Status for {}: {}", origin_url, res.status());

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    println!("HTML: {}", &body[0..40]);
    let f = fs::File::create(String::from("../") + year + "/day" + day + ".txt");
    match f {
        Ok(file) => {
            let mut b = BufWriter::new(file);
            let _ = b.write(&body.as_bytes());
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}
