#![feature(let_else)]

use std::{collections::HashMap, io::Write};

pub fn write_results_csv(results: Vec<Vec<&str>>, output: &mut impl Write) {
    for result in results {
        assert_eq!(result.len(), 5);
        for (idx, word) in result.iter().enumerate() {
            if idx != 0 {
                write!(output, ", ").unwrap();
            }
            write!(output, "{word}").unwrap();
        }
        writeln!(output).unwrap()
    }
}

pub fn solve(words: Vec<&str>) -> Vec<Vec<&str>> {
    let mut masks_to_words: HashMap<u32, Vec<&str>> = HashMap::new();
    for word in words {
        if word.len() != 5 {
            continue;
        }
        let Some(mask) = get_mask(word) else {
            continue;
        };
        if let Some(entry) = masks_to_words.get_mut(&mask) {
            // The key was already present (there exists an anagram of this word)
            entry.push(word)
        } else {
            masks_to_words.insert(mask, vec![word]);
        }
    }
    let mut masks: Vec<_> = masks_to_words.keys().cloned().collect();

    let mut solver = StepSolver::new(&mut masks, 5);
    let mut masks_results = vec![];

    solver.next_step(&mut vec![], &mut masks_results);

    masks_results
        .iter()
        .map(|result| compile_results(result, &masks_to_words))
        .flatten()
        .collect()
}

/// Converts the masks returned from the solver back to strings. Also handles anagrams.
fn compile_results<'a>(masks: &[u32], words: &HashMap<u32, Vec<&'a str>>) -> Vec<Vec<&'a str>> {
    if masks.is_empty() {
        vec![]
    } else {
        let matches = words.get(&masks[0]).unwrap();
        if masks.len() == 1 {
            matches.iter().map(|&m| vec![m]).collect()
        } else {
            let sub_results = compile_results(&masks[1..], words);
            assert!(!sub_results.is_empty());

            // This codepath does too much cloning, but it's not hot enough to matter.
            let mut results = vec![];
            for m in matches {
                let mut cloned_results = sub_results.clone();
                for result in &mut cloned_results {
                    result.push(m);
                }
                results.append(&mut cloned_results);
            }
            results
        }
    }
}

type LettersMask = u32;

/// Sets a bit for every letter present. If a letter is present multiple times, returns None.
fn get_mask(word: &str) -> Option<u32> {
    let mut mask = 0;
    for byte in word.bytes() {
        let offset = byte - b'a';
        assert!(offset < 26);
        let char_mask = 1 << offset;
        if mask & char_mask != 0 {
            return None;
        }
        mask |= char_mask;
    }
    Some(mask)
}

struct StepSolver<'a> {
    words: &'a mut Vec<LettersMask>,
    previous_letters: LettersMask,
    requested_len: usize,
}

impl<'a> StepSolver<'a> {
    fn new(words: &'a mut Vec<LettersMask>, requested_len: usize) -> Self {
        Self {
            words,
            previous_letters: 0,
            requested_len,
        }
    }
    fn next_step(
        &mut self,
        recycled_vecs: &mut Vec<Vec<LettersMask>>,
        results: &mut Vec<Vec<u32>>,
    ) {
        if self.requested_len == 0 {
            results.push(vec![]);
            return;
        }
        if self.words.len() < self.requested_len {
            return;
        }

        let mut recycled_vec = recycled_vecs.pop().unwrap_or_else(|| vec![]);

        for (idx, &word) in self.words.iter().enumerate() {
            let new_mask = self.previous_letters | word;

            // reusing previous allocations helps performance quite a bit
            recycled_vec.clear();
            recycled_vec.extend(
                self.words[idx..]
                    .iter()
                    .filter(|&&w| w & new_mask == 0)
                    .cloned(),
            );

            let mut next_solver = StepSolver {
                words: &mut recycled_vec,
                previous_letters: new_mask,
                requested_len: self.requested_len - 1,
            };

            // add the current word to all newly added results
            let previous_results_len = results.len();
            next_solver.next_step(recycled_vecs, results);
            for result in &mut results[previous_results_len..] {
                result.push(word);
            }
        }

        recycled_vecs.push(recycled_vec);
    }
}
