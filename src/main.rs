#![feature(entry_and_modify)]
#![feature(use_extern_macros)]
extern crate clap;
extern crate ngrams;

use ngrams::Ngram;
use std::collections::hash_map::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{stdin, Read};

fn histogram<C: IntoIterator<Item = T>, T: Eq + Hash>(collection: C) -> HashMap<T, usize> {
    let mut result = HashMap::new();
    for element in collection {
        result.entry(element).and_modify(|c| *c += 1).or_insert(1);
    }
    result
}

fn sort_by_value_rev<C: IntoIterator<Item = (K, V)>, K, V: Ord>(hash_map: C) -> Vec<(K, V)> {
    let mut entries: Vec<(K, V)> = hash_map.into_iter().collect();
    entries.sort_by(|&(_, ref v0), &(_, ref v1)| v0.cmp(v1).reverse());
    entries
}

fn main() {
    let matches = clap::App::new("ngRSm")
        .version("0.2.0")
        .author("Kierán Meinhardt <kieran.meinhardt@gmail.com>")
        .about("Reads in text from a file and creates n-gram statistics.")
        .arg_from_usage(
            "[size] -s --size=<N> 'Specify the length of the n-grams to analyse (default 3)'",
        )
        .arg_from_usage("[file] -f --file=<FILE> 'Specify the file to read from (default stdin)'")
        .get_matches();
    let ngram_size = clap::value_t!(matches.value_of("size"), usize).unwrap_or(3);

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

    let ngrams = input.split_whitespace().ngrams(ngram_size);

    for (k, v) in sort_by_value_rev(histogram(ngrams)) {
        println!("{}\t{}", v, k.join(" "));
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_histogram() {
        assert!(::histogram(&[] as &[i32]).is_empty());

        let statistics = ::histogram(&[1, 2, 3, 4, 1, 1, 2]);
        assert_eq!(statistics.get(&1), Some(&3));
        assert_eq!(statistics.get(&2), Some(&2));
        assert_eq!(statistics.get(&3), Some(&1));
        assert_eq!(statistics.get(&4), Some(&1));
        assert_eq!(statistics.get(&5), None);
    }

    #[test]
    fn test_sort_by_value_ref() {
        assert!(::sort_by_value_rev(HashMap::new() as HashMap<i32, i32>).is_empty());

        // example from <https://doc.rust-lang.org/std/collections/hash_map/struct.HashMap.html>
        let timber_resources: HashMap<&str, i32> =
            [("Iceland", 10), ("Norway", 100), ("Denmark", 50)]
                .iter()
                .cloned()
                .collect();
        assert_eq!(
            ::sort_by_value_rev(timber_resources),
            vec![("Norway", 100), ("Denmark", 50), ("Iceland", 10)]
        );
    }
}
