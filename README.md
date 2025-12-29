# CodeWars-First-non-repeating-character-5-kyu---Passed
Write a function named first_non_repeating_letter† that takes a string input, and returns the first character that is not repeated anywhere in the string.

For example, if given the input 'stress', the function should return 't', since the letter t only occurs once in the string, and occurs first in the string.

As an added challenge, upper- and lowercase letters are considered the same character, but the function should return the correct case for the initial letter. For example, the input 'sTreSS' should return 'T'.

If a string contains all repeating characters, it should return None.

† Note: the function is called firstNonRepeatingLetter for historical reasons, but your function should handle any Unicode character.


#[cfg(test)]
mod tests {
    use super::first_non_repeating;

    fn assert_first_non_repeating(s: &str, exp: Option<char>) {
        assert_eq!(
            first_non_repeating(s),
            exp,
            "\nYour result (left) did not match the expected output (right), given {s:?}",
        )
    }

    mod _1_fixed_tests {
        use super::*;

        #[test]
        fn _1_simple_cases() {
            assert_first_non_repeating("a", Some('a'));
            assert_first_non_repeating("stress", Some('t'));
            assert_first_non_repeating("moonmen", Some('e'));
        }

        #[test]
        fn _2_empty_string() {
            assert_first_non_repeating("", None);
        }

        #[test]
        fn _3_all_repeating() {
            assert_first_non_repeating("abba", None);
            assert_first_non_repeating("aa", None);
        }

        #[test]
        fn _4_strange_chars() {
            assert_first_non_repeating("∞§ﬁ›ﬂ∞§", Some('ﬁ'));
            assert_first_non_repeating("hello world, eh?", Some('w'));
        }

        #[test]
        fn _5_case_insensitivity() {
            assert_first_non_repeating("sTreSS", Some('T'));
            assert_first_non_repeating("Go hang a salami, I'm a lasagna hog!", Some(','));
            assert_first_non_repeating("Who is my widdle silly mopy doggy then?", Some('p'));
        }
    }

    mod _2_random_tests {
        use super::*;
        use rand::prelude::*;

        #[test]
        fn _6_random_ascii_tests() {
            let mut rng = thread_rng();
            for _ in 0..50 {
                let s = generate_random_ascii_input(&mut rng);
                let exp = reference_solution(&s);
                assert_first_non_repeating(&s, exp);
            }
        }

        #[test]
        fn _7_random_unicode_tests() {
            let mut rng = thread_rng();
            for _ in 0..50 {
                let s = generate_random_unicode_input(&mut rng);
                let exp = reference_solution(&s);
                assert_first_non_repeating(&s, exp);
            }
        }

        fn generate_random_ascii_input<R: Rng>(rng: &mut R) -> String {
            const BASE: &[u8] =
                b"abcdefghijklmnopqrstuvwxyz ABCDEFGHIJKLMNOPQRSTUVWXYZ 0123456789;,:.";
            let len = rng.gen_range(10..=60);
            let sample = if rng.gen_bool(0.4) {
                // Force None by generating len / 2 unique characters that are then
                // doubled to produce output without a single unique character. For
                // odd lengths one of the characters is added so there are 3 of that
                // single value. The vector is then shuffled.
                let mut sample = Vec::from_iter(BASE.choose_multiple(rng, len).cloned());
                sample.copy_within(0..len / 2, len / 2);
                if len % 2 == 1 {
                    *sample.last_mut().unwrap() = sample[0];
                }
                sample.shuffle(rng);
                sample
            } else {
                // Generate `len`` random characters; no guarantees are made about there being
                // unique values.
                (0..len)
                    .map(|_| BASE[rng.gen_range(0..BASE.len())])
                    .collect()
            };
            unsafe {
                // safety: only inserts ASCII characters
                String::from_utf8_unchecked(sample)
            }
        }

        fn generate_random_unicode_input<R: Rng>(rng: &mut R) -> String {
            const RANGE: std::ops::RangeInclusive<char> = '\u{2800}'..='\u{28FF}'; // Braille patterns
            let len = rng.gen_range(10..=60);
            String::from_iter(if rng.gen_bool(0.4) {
                // Force None by generating len / 2 unique characters that are then
                // doubled to produce output without a single unique character. For
                // odd lengths one of the characters is added so there are 3 of that
                // single value. The vector is then shuffled.
                let mut sample = RANGE.choose_multiple(rng, len);
                sample.copy_within(0..len / 2, len / 2);
                if len % 2 == 1 {
                    *sample.last_mut().unwrap() = sample[0];
                }
                sample.shuffle(rng);
                sample
            } else {
                (0..len).map(|_| RANGE.choose(rng).unwrap()).collect()
            })
        }

        fn reference_solution(s: &str) -> Option<char> {
            use std::collections::HashMap;
            let mut counter: HashMap<char, usize> = HashMap::new();
            for c in s.chars() {
                *counter.entry(c.to_ascii_lowercase()).or_default() += 1;
            }
            for c in s.chars() {
                let count = *counter.get(&c.to_ascii_lowercase()).unwrap_or(&0);
                if count == 1 {
                    return Some(c);
                }
            }
            None
        }
    }
}



BEST SOLUTION
use itertools::Itertools;

pub fn first_non_repeating(s: &str) -> Option<char> {
    let h = s.to_ascii_lowercase().chars().counts();
    s.chars().filter(|c| h[&c.to_ascii_lowercase()] == 1).next()
}
