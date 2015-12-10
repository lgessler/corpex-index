// The FST-based data structure we're using
extern crate fst;
use fst::{IntoStreamer, Streamer, Set, SetBuilder, Regex};

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

// For timing queries
extern crate time;

/* * * * * * * * * 
 * Main function *
 * * * * * * * * */
fn main() {
    // Pull out command-line arguments
    let mut argv = vec!();
    for argument in env::args() {
        &mut argv.push(argument);
    }
    assert!(argv.len() > 2, "Run with \"build <src file> <dst file>\" or \"run <set file>[ -p <port>]\"!");

    // Build set if in build mode, otherwise run with a pre-built set
    if argv[1] == "build".to_string() {
        build_set(&argv[2], &argv[3]).unwrap();
    } 
    else if argv[1] == "run".to_string() {
        if argv[3] == "-p".to_string() {
            accept_search(&argv[2], &argv[4]).unwrap();
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
    // Load the set that was built from build_set() into memory
    let set = Set::from_path(filename).unwrap();
    
    let mut server = Nickel::new();
    // Route post requests to / this handler function
    server.post("/", middleware! { |req, mut res| 
        let start_time = time::precise_time_ns();

        // Accept the POST data and load into struct
        let rex = req.json_as::<RegexString>().unwrap();
        println!("Received request for {}", &rex.val);

        // Add trailing and leading wildcards because corpex assumes these are present
        let search_expression = format!("{}{}{}", ".*", &rex.val, ".*");
        
        // Build regex object from our formatted regex string
        let re = Regex::new(&search_expression).unwrap();

        // Search the set with the regex object and stream the results into a vec
        let mut stream = set.search(&re).into_stream();
        let mut keys = vec![];
        while let Some(k) = stream.next() {
            keys.push(k.to_vec());
        }

        // Begin building the response JSON, as a string
        // Commented version is for pretty printing--but since the computer 
        // doesn't care, we exclude newlines and spaces.
        /*
        let mut payload = String::from("{\n  'results': [\n");
        let len = keys.len();
        for i in 0..len {
            if i < len - 1 {
                let line = format!("    '{}',\n", String::from_utf8_lossy(&keys[i]));
                payload.push_str(&line);
            } else {
                let line = format!("    '{}'\n  ]\n}}", String::from_utf8_lossy(&keys[i]));
                payload.push_str(&line);
            }
        }
        */
        let mut payload = String::from("{\"results\":[");
        let len = keys.len();
        for i in 0..len {
            if i < len - 1 {
                let line = format!("\"{}\",", String::from_utf8_lossy(&keys[i]));
                payload.push_str(&line);
            } else {
                let line = format!("\"{}\"", String::from_utf8_lossy(&keys[i]));
                payload.push_str(&line);
            }
        }
        payload.push_str("]}");

        // Tell them the response is a JSON, and send it back. (Rust returns
        // implicitly, like Ruby or CoffeeScript.)
        res.set(MediaType::Json);

        println!("Responded to {} in {} ms", &rex.val, 
                 (time::precise_time_ns() - start_time)/1000000);
        payload
    });
    
    let ip_and_string = "0.0.0.0:".to_string() + port;
    // only a &str makes this method happy, so slice the String
    server.listen(&ip_and_string[..]);
    Ok(())
}

// Given a csv_filename, build an fst::Set and write it to set_filename 
fn build_set(csv_filename: &str, set_filename: &str) -> Result<(), Error> {
    // Initialize CSV reader and set delimiter to tab.
    let rdr = csv::Reader::from_file(csv_filename).unwrap();
    let mut rdr = rdr.delimiter(b'\t');

    // Open a buffered file writer at set_filename. Give the writer to an 
    // fst::SetBuilder object, which lets you stream the set onto the disk
    // instead of keeping it all in memory.
    let wtr = io::BufWriter::new(File::create(set_filename).unwrap());
    let mut build = SetBuilder::new(wtr).unwrap();

    // Load all the lines of the CSV into memory. Only keep the third one,
    // which is the actual text in HMC's case. (The first two are metadata.)
    // Put them into a vec.
    let mut vec = Vec::new();
    for record in &mut rdr.decode() {
        let (_, _, s3): (String, String, Option<String>) = record.unwrap();
        match s3 {
            Some(s3) => vec.push(s3),
            _ => continue,
        }
    }
    
    // Sort the vec. This is necessary for fst's SetBuilder to work properly.
    vec.sort();

    // Finally, use the SetBuilder object to construct the set one line at a time.
    // Keep track of lines and discard any duplicates.
    let mut already_inserted = Vec::new();
    for line in vec {
        if already_inserted.contains(&line) {
            continue;
        }
        build.insert(&line).unwrap();
        already_inserted.push(line);
    }
    // Block until construction is finished
    build.finish().unwrap();
    Ok(())
}


