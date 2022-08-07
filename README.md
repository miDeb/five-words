# five-words

Prints the list of combinations consisting of five english five-letter words where every letter is unique.
On my machine this takes about 3 seconds.

## Usage

- Make sure you have the [rust programming language](https://www.rust-lang.org/) installed.
- Build the binary using `cargo b --release`. This will build a binary and save it to `target/release/five-words`.
- Run it using `./target/release/five-words <wordlist>` (e.g. `./target/release/five-words wordlist.txt`).

## Limitations

Due to performance optimizations this will only work for wordlists that contain only standard ascii letters (a-z).
