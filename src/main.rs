use std::env;
use std::fs::File;
use std::io::{Read, stdin};
use std::iter::Iterator;

use colored::*;

fn grep(pattern: &str, file: &mut dyn Read, action: &mut dyn FnMut(Match) -> ()) {

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    contents
        .lines()
        .flat_map(|l| matches(l, pattern))
        .for_each(action)
}

#[derive(Debug, PartialEq, Eq)]
struct Match {
    line: String,
    hit: (usize, usize), // starts and ends of each match
    // FIXME multiple hits in the same line
}

fn matches(line: &str, pattern: &str) -> Option<Match> {
    match line.find(pattern) {
        Some(start) => Some(Match{
            line: line.to_string(),
            hit: (start, (start + pattern.len())),
        }),
        None => None,
    }
}

fn print_match(match_: Match) {
    let ln = match_.line;
    let hit = match_.hit;

    println!("{}{}{}",
        &ln[0..hit.0],
        &ln[hit.0..hit.1].red(),
        &ln[hit.1..ln.len()],
    )
}

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if let [_, pattern, ref filename] = args.as_slice() {
        let mut file = File::open(filename).expect("Cannot open file");
       grep(pattern, &mut file, &mut print_match)
    } else if let [_, pattern] = args.as_slice() {
        grep(pattern, &mut stdin(), &mut print_match)
    } else {
        panic!("wrong number of arguments")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_matches() {
        assert_eq!(matches("hello world", "earth"), None);
        assert_eq!(matches("hello world", "world"), Some(Match {
            line: String::from("hello world"),
            hit: (6, 11),
        }));
    }

    #[test]
    fn test_grep() {
        let mut results: Vec<Match> = vec![];

        let mut file = Cursor::new("hello world\nrandom stuff\nworld is nice".as_bytes());

        grep("world", &mut file, &mut |m| results.push(m));

        assert_eq!(results, vec![
            Match {
                line: String::from("hello world"),
                hit: (6, 11),
            },
            Match {
                line: String::from("world is nice"),
                hit: (0, 5),
            },
        ])
    }
}

