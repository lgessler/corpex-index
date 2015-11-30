# corpex-index

This is a microservice for [lgessler/corpex](http://github.com/lgessler/corpex).
It provides a JSON API endpoint wrapped around the [rust crate
`fst`](http://blog.burntsushi.net/transducers/), a fast
regular expression matching engine. 

# Setup

Install rust: 

    curl -sSf https://static.rust-lang.org/rustup.sh | sh

Clone this repo and build:
    
    git clone git@github.com:lgessler/corpex-index.git
    cd corpex-index/src
    cargo build
    cargo run

To test, run the Python script:

    python3 debug.py

# Known issues

* Duplicate keys not allowed
* #Keys may overflow range of vals (unique u64)
