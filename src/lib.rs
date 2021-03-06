#[macro_use(hashmap)]
extern crate maplit;

use std::collections::hash_map::HashMap;
use std::hash::Hash;

/// The `histogram` function generates frequency statistics based one an iterable collection. The
/// resulting hash-map contains the element as key and its frequency as value.
///
/// ```
/// assert_eq!(ngrsm::histogram(&[1, 1]).get(&1), Some(&2));
/// ```
pub fn histogram<C: IntoIterator<Item = T>, T: Eq + Hash>(collection: C) -> HashMap<T, usize> {
    let mut result = HashMap::new();
    for element in collection {
        *result.entry(element).or_insert(0) += 1;
    }
    result
}

/// The `sort_by_value_rev`, like the name implies, sorts an iterable collection of key value pairs
/// by their values, putting the pair with the largest value up front.
///
/// ```
/// # use std::collections::HashMap;
/// let input: HashMap<_, _> = [('a', 4), ('b', -1), ('c', 0), ('d', 5)].iter().cloned().collect();
/// let output = vec![('d', 5), ('a', 4), ('c', 0), ('b', -1)];
/// assert_eq!(ngrsm::sort_by_value_rev(input), output)
/// ```
pub fn sort_by_value_rev<C, K, V>(hash_map: C) -> Vec<(K, V)>
where
    C: IntoIterator<Item = (K, V)>,
    V: Ord,
{
    let mut entries: Vec<(K, V)> = hash_map.into_iter().collect();
    entries.sort_by(|&(_, ref v0), &(_, ref v1)| v0.cmp(v1).reverse());
    entries
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn test_histogram() {
        assert!(histogram(Vec::<u8>::new()).is_empty());

        let statistics = histogram(vec![1, 2, 3, 4, 1, 1, 2]);
        assert_eq!(hashmap!{ 1 => 3, 2 => 2, 3 => 1, 4 => 1 }, statistics);
        assert_eq!(statistics.get(&5), None);
    }

    #[test]
    fn test_sort_by_value_ref() {
        let empty: HashMap<u8, u8> = HashMap::new();
        assert!(sort_by_value_rev(empty).is_empty());

        // example from <https://doc.rust-lang.org/std/collections/hash_map/struct.HashMap.html>
        let timber_resources: HashMap<_, _> = hashmap! {
            "Iceland" => 10,
            "Norway" => 100,
            "Denmark" => 50,
        };
        assert_eq!(
            sort_by_value_rev(timber_resources),
            vec![("Norway", 100), ("Denmark", 50), ("Iceland", 10)]
        );
    }
}
