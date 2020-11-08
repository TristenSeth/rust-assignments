/* The following exercises were borrowed from Will Crichton's CS 242 Rust lab. */

use std::collections::HashSet;

fn main() {
    println!("Hi! Try running \"cargo test\" to run tests.");
}

fn add_n(v: Vec<i32>, n: i32) -> Vec<i32> {
    //create new mutable vec to return
    let mut new_vec: Vec<i32> = Vec::new();

    for num in v.iter() {
        new_vec.push(num + n);
    }

    //return the new vector
    new_vec

}

fn add_n_inplace(v: &mut Vec<i32>, n: i32) {
    for i in 0..v.len() {
        v[i] += n;
    }

}

fn dedup(v: &mut Vec<i32>) {
    //create hashset to keep track of duplicates
    let mut seen: HashSet<i32> = HashSet::new();
    let mut i = 0;
    while i < v.len() {
        if seen.contains(&v[i]) {
            //remove dup
            v.remove(i);
        } else {
            //add it to list of seen values
            seen.insert(v[i]);
            i += 1;
        }
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_n() {
        assert_eq!(add_n(vec![1], 2), vec![3]);
    }

    #[test]
    fn test_add_n_inplace() {
        let mut v = vec![1];
        add_n_inplace(&mut v, 2);
        assert_eq!(v, vec![3]);
    }

    #[test]
    fn test_dedup() {
        let mut v = vec![3, 1, 0, 1, 4, 4];
        dedup(&mut v);
        assert_eq!(v, vec![3, 1, 0, 4]);
    }
}
