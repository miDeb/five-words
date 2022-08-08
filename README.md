# five-words

Prints the list of combinations consisting of five english five-letter words where every letter is unique.
On my machine this takes about 0.3 seconds.

## Usage

- Make sure you have the [rust programming language](https://www.rust-lang.org/) installed.
- Build the binary using `cargo b --release`. This will build a binary and save it to `target/release/five-words`.
- Run it using `./target/release/five-words <wordlist>` (e.g. `./target/release/five-words wordlist.txt`).

## Limitations

Due to performance optimizations this will only work for wordlists that contain only standard ascii letters (a-z).

## Implementation

This is basically a DFS (depth-first-search) implementation with some optimizations.
Instead of handling words, i.e. strings of characters, we use bitmasks to encode which letters are present and which aren't.


The rest of the optimizations applied are pretty standard:

To reduce the work needed for allocating/reallocating memory we can apply some classic optimizations and re-use some buffers.

Also, we take advantage of multiple threads to do work in parallel.
