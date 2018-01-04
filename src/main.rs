#![feature(entry_and_modify)]
#![feature(use_extern_macros)]
extern crate clap;
extern crate ngrams;

use ngrams::Ngram;
use std::collections::hash_map::HashMap;
use std::hash::Hash;
use std::io::{stdin, Read};

fn histogram<C: IntoIterator<Item = T>, T: Eq + Hash>(collection: C) -> HashMap<T, usize> {
    let mut result = HashMap::new();
    for element in collection {
        result.entry(element).and_modify(|c| *c += 1).or_insert(1);
    }
    result
}

fn main() {
    let matches = clap::App::new("ngRSm")
        .version("0.1.1")
        .author("Kierán Meinhardt <kieran.meinhardt@gmail.com>")
        .about("Reads in text from stdin and creates n-gram statistics.")
        .arg_from_usage("[size] 'Specify the length of the n-grams to analyse'")
        .get_matches();
    let ngram_size = clap::value_t!(matches.value_of("size"), usize).unwrap_or(3);

    let mut input = String::new();
    stdin().read_to_string(&mut input).expect("Could not read from stdin.");

    let ngrams = input.split_whitespace().ngrams(ngram_size);
    let statistics = histogram(ngrams);
    let mut entries: Vec<(Vec<&str>, usize)> = statistics.into_iter().collect();

    entries.sort_by(|&(_, v0), &(_, v1)| v0.cmp(&v1).reverse());

    for (k, v) in entries {
        println!("{}\t{}", v, k.join(" "));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_histogram() {
        let statistics = ::histogram(&[1, 2, 3, 4, 1, 1, 2]);
        assert_eq!(statistics.get(&1), Some(&3));
        assert_eq!(statistics.get(&2), Some(&2));
        assert_eq!(statistics.get(&3), Some(&1));
        assert_eq!(statistics.get(&4), Some(&1));
        assert_eq!(statistics.get(&5), None);
    }
}
