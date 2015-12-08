// The FSM-based data structure we're using
extern crate fst;
use fst::{IntoStreamer, Streamer, Map, MapBuilder, Regex};

// For reading CSV's and doing other file IO
extern crate csv;
use std::fs::File;
use std::io;
use std::io::Error;

// For the web service
#[macro_use] extern crate nickel;
extern crate rustc_serialize;
use nickel::{Nickel, HttpRouter, JsonBody, MediaType};

// For command-line arguments
use std::env;

/* * * * * * * * * 
 * Main function *
 * * * * * * * * */
fn main() {
    // Pull out command-line arguments
    let mut argv = vec!();
    for argument in env::args() {
        &mut argv.push(argument);
    }
    assert!(argv.len() > 2, "Run with \"build <src file> <dst file>\" or \"run <map file>[ -p <port>]\"!");

    // Build map if in build mode, otherwise run with a pre-built map
    if argv[1] == "build".to_string() {
        build_map(&argv[2], &argv[3]).unwrap();
    } 
    else if argv[1] == "run".to_string() {
        if argv[2] == "-p".to_string() {
            accept_search(&argv[2], &argv[3]).unwrap();
        } else {
            accept_search(&argv[2], &"6767".to_string()).unwrap();    
        }
    } 
    else {
        println!("Unknown command {}. Use \"build\" or \"run\".", &argv[1]);
        panic!("Unknown command!");
    }
}

// Struct for parsing the JSON request in accept_search
#[derive(RustcDecodable, RustcEncodable)]
struct RegexString {
    val: String,
}

// Starts a web service that will take POST requests with a RegexString 
// struct-compatible JSON inside. Responds with a list of matched strings 
// and their correspondng values.
fn accept_search(filename: &String, port: &String) -> Result<(), Error> {
    // Load the map that was built from build_map() into memory
    let map = Map::from_path(filename).unwrap();
    
    let mut server = Nickel::new();
    // Route post requests to / this handler function
    server.post("/", middleware! { |req, mut res| 
        // Accept the POST data and load into struct
        let rex = req.json_as::<RegexString>().unwrap();

        println!("Received request for {}", &rex.val);
        // Add trailing and leading wildcards because corpex assumes these are present
        let search_expression = format!("{}{}{}", ".*", &rex.val, ".*");
        
        // Build regex object from our formatted regex string
        let re = Regex::new(&search_expression).unwrap();

        // Search the map with the regex object and stream the results into a vec
        let mut stream = map.search(&re).into_stream();
        let mut kvs = vec![];
        while let Some((k, v)) = stream.next() {
            kvs.push((k.to_vec(), v));
        }

        // Begin building the response JSON, as a string
        let mut payload = String::from("{\n");
        for (k, v) in kvs {
            let line = format!("'{}': '{}'", String::from_utf8_lossy(&k), v);
            payload.push_str(&line);
        }
        payload.push_str("}");

        // Tell them the response is a JSON, and send it back. (Rust returns
        // implicitly, like Ruby or CoffeeScript.)
        res.set(MediaType::Json);
        payload
    });
    
    let ip_and_string = "0.0.0.0:".to_string() + port;
    // only a &str makes this method happy, so slice the String
    server.listen(&ip_and_string[..]);
    Ok(())
}

// Given a CSV at filename, build an fst::Map and write it to mapname
fn build_map(filename: &str, mapname: &str) -> Result<(), Error> {
    // Initialize CSV reader and set delimiter to tab.
    let rdr = csv::Reader::from_file(filename).unwrap();
    let mut rdr = rdr.delimiter(b'\t');

    // Open a buffered file writer at mapname. Give the writer to an 
    // fst::MapBuilder object, which lets you stream the map onto the disk
    // instead of keeping it all in memory.
    let wtr = io::BufWriter::new(File::create(mapname).unwrap());
    let mut build = MapBuilder::new(wtr).unwrap();

    // Load all the lines of the CSV into memory. Only keep the third one,
    // which is the actual text in HMC's case. (The first two are metadata.)
    // Put them into a vec.
    let mut vec = Vec::new();
    let mut i = 0;
    for record in &mut rdr.decode() {
        let (_, _, s3): (String, String, Option<String>) = record.unwrap();
        i += 1;
        match s3 {
            Some(s3) => vec.push((s3, i)),
            _ => continue,
        }
    }
    
    // Sort the vec. This is necessary for fst's MapBuilder to work properly.
    vec.sort();

    // Finally, use the MapBuilder object to construct the map one line at a time.
    // Keep track of lines and discard any duplicates.
    let mut already_inserted = Vec::new();
    for tuple in vec {
        if already_inserted.contains(&tuple.0) {
            continue;
        }
        build.insert(&tuple.0, tuple.1).unwrap();
        already_inserted.push(tuple.0);
    }
    // Block until construction is finished
    build.finish().unwrap();
    Ok(())
}


