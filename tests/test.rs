use five_words::solve;

#[test]
fn test_expected() {
    let wordlist = std::fs::read_to_string("wordlist.txt").unwrap();
    let words = wordlist.lines().collect();

    let results = solve(words);

    assert_eq!(results.len(), 831);
}
