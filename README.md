# corpex-index

This is a microservice for [lgessler/corpex](http://github.com/lgessler/corpex).
It provides a JSON API endpoint wrapped around the [rust crate
`fst`](http://blog.burntsushi.net/transducers/), a fast
regular expression matching engine (among other things).

# Setup

Install rust: 

    curl -sSf https://static.rust-lang.org/rustup.sh | sh

Clone this repo and build:
    
    git clone git@github.com:lgessler/corpex-index.git
    cd corpex-index/src
    cargo build --release
    target/release/corpex-index build <hindmonocorp05.plaintext file path> set.fst
    target/release/corpex-index run set.fst

To test, run the Python script:

    python3 scripts/debug.py

*Todo:* add instructions on sharding
