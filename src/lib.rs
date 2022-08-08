#![feature(let_else)]

use std::{collections::HashMap, io::Write, thread};

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
    let masks: Vec<_> = masks_to_words.keys().cloned().collect();

    let threads = thread::available_parallelism()
        .unwrap_or(2.try_into().unwrap())
        .into();

    thread::scope(|scope| {
        (0..threads)
            .map(|i| {
                let masks = &masks;
                let masks_to_words = &masks_to_words;
                scope.spawn::<_, Vec<Vec<&str>>>(move || {
                    let mut masks_results = vec![];
                    let mut solver = StepSolver {
                        words: &masks[masks.len() * i / threads..],
                        previous_letters: 0,
                        requested_len: 5,
                        recycled_vecs: &mut vec![],
                        results: &mut masks_results,
                        first_step_end_idx: (masks.len() * (i + 1) / threads)
                            - (masks.len() * i / threads),
                    };

                    solver.next_step();

                    masks_results
                        .iter()
                        .map(|result| compile_results(result, &masks_to_words))
                        .flatten()
                        .collect()
                })
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .reduce(|mut a, mut b| {
                a.append(&mut b);
                a
            })
            .unwrap_or(vec![])
    })
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

struct StepSolver<'a, 'b> {
    words: &'a [LettersMask],
    previous_letters: LettersMask,
    requested_len: usize,
    recycled_vecs: &'b mut Vec<Vec<LettersMask>>,
    results: &'b mut Vec<Vec<u32>>,
    first_step_end_idx: usize,
}

impl<'a, 'b> StepSolver<'a, 'b> {
    fn next_step(&mut self) {
        if self.requested_len == 0 {
            self.results.push(vec![]);
            return;
        }
        if self.words.len() < self.requested_len {
            return;
        }

        let mut recycled_vec = self.recycled_vecs.pop().unwrap_or_else(|| vec![]);

        for (idx, &word) in self.words[..self.first_step_end_idx].iter().enumerate() {
            let new_mask = self.previous_letters | word;

            // reusing previous allocations helps performance quite a bit
            recycled_vec.clear();
            recycled_vec.extend(
                self.words[idx..]
                    .iter()
                    .filter(|&&w| w & new_mask == 0)
                    .cloned(),
            );

            let previous_results_len = self.results.len();

            let words_len = recycled_vec.len();

            let mut next_solver = StepSolver {
                words: &mut recycled_vec,
                previous_letters: new_mask,
                requested_len: self.requested_len - 1,
                recycled_vecs: self.recycled_vecs,
                results: self.results,
                first_step_end_idx: words_len,
            };

            // add the current word to all newly added results
            next_solver.next_step();
            for result in &mut self.results[previous_results_len..] {
                result.push(word);
            }
        }

        self.recycled_vecs.push(recycled_vec);
    }
}
