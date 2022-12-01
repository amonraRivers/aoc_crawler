use reqwest::{self, header::HeaderValue};
use std::fs;
use std::io::{BufWriter, Read, Write};
fn main() {
    let client = reqwest::blocking::Client::new();
    let base_url = String::from("https://adventofcode.com/2021/day/");
    let day = "1";
    let origin_url = base_url + day + "/input";
    let mut headers = reqwest::header::HeaderMap::new();
    let cookie="_ga=GA1.2.2064259430.1669580633; _gid=GA1.2.379236820.1669580633; session=53616c7465645f5f6461ca7ca6488e83e8ebaad6a16c9d41bcd38fd091cfde66041bab5e4f1d7d1e32fb7d94794f3995d38ff2e4252165e5f48713378f6053c9";
    let hv = HeaderValue::from_str(cookie).unwrap();
    headers.append(reqwest::header::COOKIE, hv);

    let mut res = client.get(&origin_url).headers(headers).send().unwrap();
    println!("Status for {}: {}", origin_url, res.status());

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    println!("HTML: {}", &body[0..40]);
    let f = fs::File::create(String::from("../adventOfCode/2022/days/") + day + ".txt");
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
