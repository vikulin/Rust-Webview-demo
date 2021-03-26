extern crate winres;

use std::fs::File;
use std::path::Path;
use std::io::Write;

fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("yggdrasil.ico");
        res.compile().unwrap();
    }

    //download_iana_zones("iana-tlds.txt", "iana-hashes.txt");
}


fn download_iana_zones(zones_name: &str, hashes_name: &str) {
    let response = minreq::get("https://data.iana.org/TLD/tlds-alpha-by-domain.txt").send().expect("Could not make request!");

    let response = response.as_str().expect("Response is not a valid UTF-8!").to_lowercase();
    let list: Vec<_> = response.split("\n").collect();
    let mut zones = String::new();
    let mut hashes = String::new();
    for string in list {
        if !string.starts_with("#") && !string.is_empty() {
            zones.push_str(string);
            zones.push('\n');

//            hashes.push_str(&hash_identity(string));
//            hashes.push('\n');
        }
    }

    match File::create(Path::new(zones_name)) {
        Ok(mut file) => {
            file.write_all(zones.trim().as_bytes()).expect("Error saving TLDs file!");
        }
        Err(e) => { println!("Error opening TLDs file!\n{}", e); }
    }

    match File::create(Path::new(hashes_name)) {
        Ok(mut file) => {
            file.write_all(hashes.trim().as_bytes()).expect("Error saving TLD-hashes file!");
        }
        Err(e) => { println!("Error opening TLD-hashes file!\n{}", e); }
    }
}

/// Convert bytes array to HEX format
pub fn to_hex(buf: &[u8]) -> String {
    let mut result = String::new();
    for x in buf.iter() {
        result.push_str(&format!("{:01$X}", x, 2));
    }
    result
}
