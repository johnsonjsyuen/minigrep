use std::env;
use std::fs::File;
use std::io::{Read, stdin};
use std::iter::Iterator;

fn grep<'a>(pattern: &'a str, file: &mut Read, action: &Fn(&str) -> ()) {

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    contents
        .lines()
        .filter(move |l| l.contains(pattern))
        .for_each(action)
}

fn print_match(match_: &str) {
    println!("{}", match_)
}

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if let [_, pattern, ref filename] = args.as_slice() {
        let mut file = File::open(filename).expect("Cannot open file");
       grep(pattern, &mut file, &print_match)
    } else if let [_, pattern] = args.as_slice() {
        grep(pattern, &mut stdin(), &print_match)
    } else {
        panic!("wrong number of arguments")
    }

}
