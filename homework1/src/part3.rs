/*
    CIS198 Homework 1
    Part 3: Ownership, move semantics, and lifetimes

    Complete and write at least one unit test for each function you implement.
    If it already has a unit test, either add assertions to it or add a new one.
    Also answer the questions in text.
*/

/*
    Problem 1: Swap ints

    Implement the function that swaps two integers, and write unit tests.

    The Rust borrow checker may help avoid some possible bugs.

    Then answer this question:
    Q: A common source of error in swap implementations is failing to work if
       the two references are the same. Why don't you need to worry about this
       case in Rust?

    A: There can be the only one mutable borrow at time. So, if we try to swap
    the same reference there will be an error: "cannot borrow `a` as mutable more
    than once at a time"

    (Try writing a unit test where they are both
    the same, i.e. swap_ints(&mut x, &mut x).)
*/
pub fn swap_ints(x1: &mut i32, x2: &mut i32) {
    *x1 ^= *x2;
    *x2 ^= *x1;
    *x1 ^= *x2;
}

#[test]
fn test_swap_ints() {
    let (mut a, mut b) = (1337, 2000);
    swap_ints(&mut a, &mut b);
    assert_eq!(a, 2000);
    assert_eq!(b, 1337);
}

/*
    Problem 2: String duplication
*/

#[test]
fn copy_string_test() {
    let str1 = String::from("foo");
    let str2 = str1.clone();
    assert_eq!(str1, str2);
}

// This test doesn't work. Fix it by copying strings properly.
// Q1. What went wrong?
// A1: A value was moved to str2 and no longer available in str1 due to String points to the
// heap.

// Q2. How come it works fine here?
// A2: Primitive data types, such as ints in the example below, are being cloned automatically.
#[test]
fn copy_int_test() {
    let i1 = 1;
    let i2 = i1;
    assert_eq!(i1, i2);
}
//
// Now implement the following function that duplicates a string n times.
pub fn duplicate_string(s: &str, times: usize) -> Vec<String> {
    (0..times).into_iter().map(|_| s.to_string()).collect::<Vec<String>>()
}

#[test]
fn test_duplicate_string() {
    assert_eq!(
        duplicate_string("a", 10),
        vec!["a", "a", "a", "a", "a", "a", "a", "a", "a", "a"]
    );
}

/*
    Problem 3: String duplication continued

    These two don't work either. Fix by changing the type of "string" in the
    function copy_me ONLY, and by adjusting the parameter to "copy_me" where
    it's called.
*/

pub fn copy_me(string: &str) -> String {
    string.to_string()
}

#[test]
fn copy_me_test() {
    let str1 = String::from("foo");
    assert_eq!(str1, copy_me(&str1));
}

#[test]
fn copy_me_test2() {
    let str1 = String::from("foo");
    let str2 = copy_me(&str1);
    assert_eq!(str1, str2);
}

/*
    Problem 4: Lifetime specifiers

    For each of the following three functions, either implement it by adding
    lifetime specifiers, or explain why this is not possible.

    (It's not truly impossible -- we will see later on that advanced features
    such as "unsafe code" can be used to turn off Rust's safety and lifetime
    checks.)
*/
// fn new_ref_string() -> &String {
//     &String::from("meow")
//     // Error: cannot return reference to temporary value. I assume, it happens due to a nature
//     // of String type.
// }

pub fn new_ref_str<'a>() -> &'a str {
    "lol"
}

// The same function from part2
pub fn pick_longest2<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() < s2.len() {
        s2
    } else {
        s1
    }
}

#[test]
fn test_lifetime_specs() {
    assert_eq!(new_ref_str(), "lol");
    assert_eq!(pick_longest2("dog", "dogs"), "dogs");
}

/*
    Problem 5: Using functions with lifetimes

    Write two versions of a function which returns the longest string in a
    vector, using pick_longest2 as a helper function.

    If the vector is empty, return "".

    Q1. In pick_longest_in_v2, if you were to explicitly specify the lifetime
        of the input and output, what should it be?

    Q2. What are the pros and cons of v1 and v2?
*/

pub fn pick_longest_in_v1(v: Vec<String>) -> String {
    v.iter().fold(String::new(), |result, value| {
        if value.len() < result.len() {
            result
        } else {
            value.clone()
        }
    })
}

pub fn pick_longest_in_v2<'a>(v: Vec<&'a str>) -> &'a str {
    v.iter().fold(
        "",
        |result, &value| {
            if value.len() < result.len() {
                result
            } else {
                value
            }
        },
    )
}

#[test]
fn test_pick_longest_in_vectors() {
    assert_eq!(
        pick_longest_in_v1(vec!["A".to_string(), "aaaa".to_string()]),
        "aaaa".to_string()
    );
    assert_eq!(pick_longest_in_v2(vec!["A", "aaaa"]), "aaaa");
}

/*
    Problem 6: Move semantics

    Write three versions of a function that pads a vector with zeros.
    Fail if the vector is larger than the desired length.

    Use .clone() if necessary to make any additional unit tests compile.

    Which of these functions do you prefer? Which is the most efficient?
*/

pub fn pad_with_zeros_v1(v: Vec<usize>, desired_len: usize) -> Vec<usize> {
    let n = desired_len - v.len();
    let result =
        v.iter().chain((0..n).map(|_| &0)).fold(Vec::new(), |mut acc, val| {
            acc.push(*val);
            acc
        });
    debug_assert_eq!(result.len(), desired_len);
    result
}

pub fn pad_with_zeros_v2(slice: &[usize], desired_len: usize) -> Vec<usize> {
    pad_with_zeros_v1(slice.to_vec(), desired_len)
}

pub fn pad_with_zeros_v3(v: &mut Vec<usize>, desired_len: usize) {
    let n = desired_len - v.len();
    (0..n).into_iter().fold(v, |v, _| {
        v.push(0 as usize);
        v
    });
}

#[test]
fn test_pad_twice_v1() {
    let v = vec![1];
    let v = pad_with_zeros_v1(v, 2);
    let v = pad_with_zeros_v1(v, 4);
    assert_eq!(v, vec![1, 0, 0, 0]);
}

#[test]
fn test_pad_twice_v2() {
    let v = vec![1];
    let v = pad_with_zeros_v2(&v, 2);
    let v = pad_with_zeros_v2(&v, 4);
    assert_eq!(v, vec![1, 0, 0, 0]);
}

#[test]
fn test_pad_twice_v3() {
    let mut v = vec![1];
    pad_with_zeros_v3(&mut v, 2);
    pad_with_zeros_v3(&mut v, 4);
    assert_eq!(v, vec![1, 0, 0, 0]);
}

/*
    Problem 7: Move semantics continued

    Write a function which appends a row to a vector of vectors.
    Notice that it takes ownership over the row.
    You shouldn't need to use .clone().

    Why is this more general than being passed a &[bool]
    and cloning it?

    Second, write a function which returns whether
    a row equals the first row in the vector of vectors.
    Notice that it does not take ownership over the row.

    Why is this more general than being passed a Vec<bool>?
*/

pub fn append_row(grid: &mut Vec<Vec<bool>>, row: Vec<bool>) {
    grid.push(row);
}

#[test]
fn test_append_row() {
    let grid = &mut vec![vec![true], vec![false]];
    let row = vec![true];
    append_row(grid, row);
    assert_eq!(grid, &mut vec![vec![true], vec![false], vec![true]]);
}

pub fn is_first_row(grid: &[Vec<bool>], row: &[bool]) -> bool {
    match grid.first() {
        Some(r) => *r == row.to_vec(),
        None => false,
    }
}

#[test]
fn test_is_first_row() {
    assert_eq!(is_first_row(&[vec![true], vec![false]], &[true]), true);
    assert_eq!(is_first_row(&[], &[true]), false);
}

/*
    Problem 8: Modifying while iterating

    In C and C++, you run into subtle bugs if you try to modify a data
    structure while iterating over it. Rust's move semantics prevents that.
*/

use std::collections::HashMap;

// To familiarize yourself with HashMaps,
// implement the following function which converts pairs from a slice
// into key-value pairs in a hashmap.
// Documentation:
// https://doc.rust-lang.org/std/collections/struct.HashMap.html

pub fn vector_to_hashmap(v: &[(i32, String)]) -> HashMap<i32, String> {
    v.iter().fold(HashMap::new(), |mut dict, (key, value)| {
        dict.insert(*key, value.to_string());
        dict
    })
}

#[test]
fn test_vector_to_hashmap() {
    let pairs = &[(1, "val1".to_string()), (2, "val2".to_string())];
    let hm = vector_to_hashmap(pairs);
    for (key, value) in pairs.into_iter() {
        match hm.get(key) {
            Some(hm_value) => assert_eq!(hm_value, value),
            None => assert!(false),
        }
    }
}

// Now rewrite this function to delete all entries in hashmap where the keys
// are negative.
pub fn delete_negative_keys(h: &mut HashMap<i32, i32>) {
    h.retain(|key, _| *key >= 0);
}

#[test]
fn test_delete_negative_keys() {
    let mut hm: HashMap<i32, i32> = HashMap::new();
    (-5..5).into_iter().for_each(|id| {
        hm.insert(id, id);
    });
    delete_negative_keys(&mut hm);
    assert_eq!(hm.into_iter().all(|(key, _)| key >= 0), true);
}

/*
    Problem 9: The Entry API

    Move semantics present interesting API design choices not found in other
    languages.
    HashMap is an example of such a API.
    Specifically, the Entry API:
    https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html

    This allows for efficient HashMap access because we only access
    the entry in the map (computing an expensive hash function) once.

    Implement a function which does the following:
        For all entries in `add`: (k, v)
        If `k` exists in `merged`, append `v` to the value of `merged[k]`.
        If that `k` doesn't exist in `merged`, add the (k, v) to `merged`.
    Use `or_insert` and `and_modify`.
*/

pub fn merge_maps(
    merged: &mut HashMap<String, String>,
    add: HashMap<String, String>,
) {
    for (key, value) in add.into_iter() {
        merged
            .entry(key)
            .and_modify(|val| val.push_str(value.as_str()))
            .or_insert(value);
    }
}

#[test]
fn test_merge_maps() {
    let mut merged: &mut HashMap<String, String> = &mut HashMap::new();
    merged.insert("a".to_string(), "1".to_string());

    let mut add: HashMap<String, String> = HashMap::new();
    add.insert("b".to_string(), "2".to_string());
    add.insert("a".to_string(), "3".to_string());

    merge_maps(&mut merged, add);

    let mut expected: HashMap<String, String> = HashMap::new();
    expected.insert("a".to_string(), "13".to_string());
    expected.insert("b".to_string(), "2".to_string());

    for (key, val) in expected.into_iter() {
        match merged.get(&key) {
            Some(merged_value) => assert_eq!(val, *merged_value),
            None => assert!(false),
        }
    }
}
