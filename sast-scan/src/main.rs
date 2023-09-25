use serde_json::de::from_reader;

use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use regex::Regex;


struct CWE{
    id: String,
    regex: String, 
}

fn main(){
    let mut file = File::open("../config.json").unwrap();
    let mut buff = String::new();

    file.read_to_string(&mut buff).unwrap();
    let config: String = serde_json::from_str(&mut buff).unwrap();
    

}