extern crate fst;
extern crate csv;
#[macro_use] extern crate nickel;
extern crate rustc_serialize;

use std::fs::File;
use std::io;
use std::io::Error;

use fst::{IntoStreamer, Streamer, Map, MapBuilder, Regex};
use nickel::{Nickel, HttpRouter, JsonBody, MediaType};
use rustc_serialize::json;

fn accept_search() -> Result<(), Error> {
    // now search over the map
    let map = Map::from_path("map.fst").unwrap();
    
    let mut server = Nickel::new();
    server.post("/", middleware! { |req, mut res| 
        let rex = req.json_as::<RegexString>().unwrap();

        println!("Received request for {}", &rex.val);
        let search_expression = format!("{}{}{}", ".*", &rex.val, ".*");
        
        let re = Regex::new(&search_expression).unwrap();
        let mut stream = map.search(&re).into_stream();

        let mut kvs = vec![];
        while let Some((k, v)) = stream.next() {
            kvs.push((k.to_vec(), v));
        }

        let mut payload = String::from("{\n");
        for (k, v) in kvs {
            let line = format!("'{}': '{}'", String::from_utf8_lossy(&k), v);
            payload.push_str(&line);
        }
        payload.push_str("}");

        res.set(MediaType::Json);
        payload
    });
    
    server.listen("127.0.0.1:6767");
    Ok(())
}


fn main() {
    build_map().unwrap();
    //search("यहा").unwrap();
    //let s = ".*यहा.*";
    //let s = ".*";
    //search(&s).unwrap();
    //accept_search().unwrap();

}

#[derive(RustcDecodable, RustcEncodable)]
struct RegexString {
    val: String,
}

fn search(regex: &str) -> Result<(), Error> {
    // now search over the map
    let map = Map::from_path("map.fst").unwrap();

    let re = Regex::new(regex).unwrap();
    let mut stream = map.search(&re).into_stream();

    let mut kvs = vec![];
    while let Some((k, v)) = stream.next() {
        kvs.push((k.to_vec(), v));
    }

    for (k, v) in kvs {
        println!("{} {}", String::from_utf8_lossy(&k), v);
    }
    Ok(())
}

fn build_map() -> Result<(), Error> {
    let rdr = csv::Reader::from_file("./hmcsample.txt").unwrap();
    let mut rdr = rdr.delimiter(b'\t');

    let wtr = io::BufWriter::new(File::create("map.fst").unwrap());
    let mut build = MapBuilder::new(wtr).unwrap();

    let mut vec = Vec::new();
    let mut i = 0;
    let total: f64 = 44486496.0;
    for record in &mut rdr.decode() {
        let (_, _, s3): (String, String, Option<String>) = record.unwrap();
        i += 1;
        print!("{}          \r", (i as f64)/total * 100.0);
        match s3 {
            Some(s3) => vec.push((s3, i)),
            _ => continue,
        }
    }
    
    vec.sort();
    let mut already_inserted = Vec::new();
    for tuple in vec {
        if already_inserted.contains(&tuple.0) {
            continue;
        }
        build.insert(&tuple.0, tuple.1).unwrap();
        already_inserted.push(tuple.0);
    }
    build.finish().unwrap();
    Ok(())
}


