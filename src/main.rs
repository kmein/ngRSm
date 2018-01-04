#![feature(use_extern_macros)]
extern crate clap;
extern crate ngrams;
extern crate ngrsm;

use ngrams::Ngram;
use ngrsm::*;
use std::fs::File;
use std::io::{stdin, Read};

fn main() {
    let matches = clap::App::new("ngRSm")
        .version("0.3.0")
        .author("Kierán Meinhardt <kieran.meinhardt@gmail.com>")
        .about("Reads in text from a file and creates n-gram statistics.")
        .arg_from_usage("<size> -s --size=<N> 'Specify the length of the n-grams to analyse'")
        .arg_from_usage("[file] -f --file=<FILE> 'Specify the file to read from (default stdin)'")
        .arg_from_usage(
            "[count] -c --count=<N> 'Specify the amount of n-grams to display (default all)'",
        )
        .arg_from_usage("[file] -f --file=<FILE> 'Specify the file to read from (default stdin)'")
        .get_matches();
    let ngram_size = clap::value_t!(matches.value_of("size"), usize).unwrap_or(1);
    let ngram_count = clap::value_t!(matches.value_of("count"), usize).ok();

    let mut input = String::new();
    match matches.value_of("file") {
        Some(path) => File::open(path)
            .expect("File not found.")
            .read_to_string(&mut input)
            .expect("Could not read from file."),
        None => stdin()
            .read_to_string(&mut input)
            .expect("Could not read from stdin."),
    };

    let words = input.split_whitespace();
    let statistics = match ngram_size {
        1 => sort_by_value_rev(histogram(words.into_iter().map(|x| vec![x]))),
        _ => sort_by_value_rev(histogram(words.ngrams(ngram_size))),
    };

    for (k, v) in match ngram_count {
        None => statistics,
        Some(c) => statistics.into_iter().take(c).collect(),
    } {
        println!("{:>6} {}", v, k.join(" "));
    }
}
