use std::cmp::Ordering;
use std::collections::HashSet;
/*
    CIS198 Homework 1
    Part 1: Implementing functions

    Complete and write at least one unit test for each function you implement.
    If it already has a unit test, either add assertions to it or add a new one.
    Also answer the questions in text.
*/

// Remove these once you are done editing the file!
// This will result in useful warnings if you missed something.
#![allow(dead_code)]
#![allow(unused_variables)]

/*
    Problem 1: Double

    Implement the function that doubles an integer in three different ways.

    What are some differences between them? Can you write unit tests
    which fail (or fail to compile) for some but not others?

    Which of the three do you prefer?
*/

pub fn double_v1(n: i32) -> i32 {
    n << 1
}

pub fn double_v2(n: &i32) -> i32 {
    n << 1
}

pub fn double_v3(n: &mut i32) {
    *n <<= 1
}

#[test]
fn test_double_v1() {
    assert_eq!(double_v1(2), 4);
    assert_eq!(double_v1(-3), -6);
}

#[test]
fn test_double_v2() {
    let n: i32 = 2;
    assert_eq!(double_v2(&n), 4);

    let n = -3;
    assert_eq!(double_v2(&n), -6);
}

#[test]
fn test_double_v3() {
    let mut n: i32 = 2;
    double_v3(&mut n);
    assert_eq!(n, 4);

    n = -3;
    double_v3(&mut n);
    assert_eq!(n, -6);
}

/*
    Problem 2: Integer square root

    Implement the integer square root function: sqrt(n) should return the
    largest m such that m * m <= n. For a 'harder' version, try to do it more
    efficiently than trying every possibility.

    Solution: used a binary search to find an answer in O(log n)
*/
pub fn sqrt(n: usize) -> usize {
    if n == 0 || n == 1 { return n; }
    let (mut lo, mut hi, mut answer): (usize, usize, usize) = (1, n, 0);
    while lo <= hi {
        let mid: usize = lo + (hi - lo) / 2;
        match n.cmp(&(mid * mid)) {
            Ordering::Equal => { return mid; },
            Ordering::Less => { hi = mid - 1; },
            Ordering::Greater => { answer = mid; lo = mid + 1; },
        }
    }
    answer
}

#[test]
fn test_sqrt() {
    assert_eq!(sqrt(10), 3);
    assert_eq!(sqrt(4), 2);
    assert_eq!(sqrt(1), 1);
    assert_eq!(sqrt(100), 10);
    assert_eq!(sqrt(99), 9);
}

/*
    Problem 3: Slice sum

    Implement the sum function on slices in two different ways
    (using different for loop patterns).
    Do not use the predefined sum function.
    Also, try to do it without an unnecessary `return` statement at the end --
    Clippy should detect if you mess this up.

    Which of the two ways do you prefer?
*/
pub fn sum_v1(slice: &[i32]) -> i32 {
    let mut sum: i32 = 0;
    for &v in slice {
        sum += v
    }
    sum
}

pub fn sum_v2(slice: &[i32]) -> i32 {
    slice.iter().fold(0, |acc, value| *value + acc)
}


#[test]
fn test_sum_v1() {
    assert_eq!(sum_v1(&[1,2,3,4,5]), 15);
    assert_eq!(sum_v1(&[1]), 1);
    assert_eq!(sum_v1(&[]), 0);
}

#[test]
fn test_sum_v2() {
    assert_eq!(sum_v2(&[1,2,3,4,5]), 15);
    assert_eq!(sum_v2(&[1]), 1);
    assert_eq!(sum_v2(&[]), 0);
}
/*
    Problem 4: Unique

    Make unique. Create a new vector which contains each item in the vector
    only once! Much like a set would.
    This doesn't need to be efficient; you can use a for loop.

    Solution: Memory O(N), Time O(N)
*/

pub fn unique(slice: &[i32]) -> Vec<i32> {
    slice
        .into_iter()
        .fold(HashSet::new(), |mut hs: HashSet<i32>, value| {
            hs.insert(*value);
            hs
        })
        .into_iter()
        .collect::<Vec<i32>>()
}


#[test]
fn test_unique() {
    assert_eq!(unique(&[1,1,1]), vec![1]);
    assert_eq!(unique(&[]), vec![]);

    let mut unique_elements = unique(&[1,1,1,2,3]);
    unique_elements.sort();
    assert_eq!(unique_elements, vec![1, 2, 3]);

    unique_elements = unique(&[1,2,3]);
    unique_elements.sort();
    assert_eq!(unique_elements, vec![1,2,3]);
}

/*
    Problem 5: Filter

    Return a new vector containing only elements that satisfy `pred`.
    This uses some unfamiliar syntax for the type of pred -- all you need
    to know is that pred is a function from i32 to bool.
*/
pub fn filter(slice: &[i32], pred: impl Fn(i32) -> bool) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for &v in slice {
        if pred(v) {
            result.push(v);
        }
    }
    result
}

#[test]
fn test_filter() {
    fn is_even(n: i32) -> bool {
        n % 2 == 0
    }
    assert_eq!(filter(&vec![1, 2, 3, 4, 5, 6], &is_even), vec![2, 4, 6]);

    fn is_odd(n: i32) -> bool {
        !is_even(n)
    }

    assert_eq!(filter(&vec![1, 2, 3, 4, 5, 6], &is_odd), vec![1, 3, 5]);
}

/*
    Problem 6: Fibonacci

    Given starting fibonacci numbers n1 and n2, compute a vector of
    length 'out_size'
    where v[i] is the ith fibonacci number.
*/
pub fn fibonacci(n1: i32, n2: i32, out_size: usize) -> Vec<i32> {
    let (mut a, mut b) = (n1.clone(), n2.clone());
    (0..out_size)
        .into_iter()
        .fold(vec![], |mut acc: Vec<i32>, _| {
            let temp = b;
            b = a + b;
            a = temp;
            acc.push(b);
            acc
    })
}

#[test]
fn test_fibonacci() {
    assert_eq!(fibonacci(1, 1, 2), vec![2, 3]);
    assert_eq!(fibonacci(1, 1, 4), vec![2, 3, 5, 8]);
    assert_eq!(fibonacci(2, 3, 4), vec![5, 8, 13, 21]);
}

/*
    Problem 7: String concatenation

    Create a function which concats 2 &strs and returns a String,
    and a function which concats 2 Strings and returns a String.

    You may use any standard library function you wish.

    What are some reasons the second function is not efficient?
*/
pub fn str_concat(s1: &str, s2: &str) -> String {
    let mut owned_string: String = s1.to_owned();
    owned_string.push_str(s2);
    owned_string
}

pub fn string_concat(s1: String, s2: String) -> String {
    str_concat(s1.as_str(), s2.as_str())
}


#[test]
fn test_str_concat() {
    let abc: &str = "abc";
    let def: &str = "def";
    assert_eq!(str_concat(abc, def), String::from("abcdef"));
}

#[test]
fn test_string_concat() {
    let abc: String = String::from("abc");
    let def: String = String::from("def");
    assert_eq!(string_concat(abc, def), String::from("abcdef"));
}

/*
    Problem 8: String concatenation continued

    Convert a Vec<String> into a String.
    Your answer to the previous part may help.
*/

pub fn concat_all(v: Vec<String>) -> String {
    v.iter().fold(String::new(), |mut out, current| {
        out.push_str(current);
        out
    })
}

#[test]
fn test_concat_all() {
    let input: Vec<String> = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    assert_eq!(concat_all(input), String::from("abc"));
}
/*
    Problem 9: Parsing

    Convert a Vec<String> into a Vec<i32> and vice versa.

    Assume all strings are correct numbers! We will do error handling later.
    Use `.expect("ignoring error")` to ignore Result from parse()
    See https://doc.rust-lang.org/std/primitive.str.html#method.parse

    The unit tests check if your functions are inverses of each other.

    A useful macro: format! is like println! but returns a String.
*/

pub fn parse_all(v: Vec<String>) -> Vec<i32> {
    v.iter().map(|value| value.parse::<i32>().expect(":)")).collect::<Vec<i32>>()
}

pub fn print_all(v: Vec<i32>) -> Vec<String> {
    v.iter().map(|value| value.to_string()).collect::<Vec<String>>()
}

#[test]
fn test_print_parse() {
    assert_eq!(parse_all(print_all(vec![1, 2])), vec![1, 2]);
}

#[test]
fn test_parse_print() {
    let v = vec!["1".to_string(), "2".to_string()];
    assert_eq!(print_all(parse_all(v.clone())), v);
}

/*
    Problem 10: Composing functions

    Implement a function which concatenates the even Fibonacci
    numbers out of the first n Fibonacci numbers.

    For example: if n = 6, the first 5 Fibonacci numbers are 1, 1, 2, 3, 5, 8,
    so the function should return the String "28".

    Don't use a for loop! Your previous functions should be sufficient.
*/

pub fn concat_even_fibonaccis(n: usize) -> String {
    unimplemented!()
}

#[test]
fn test_concat_even_fibonaccis() {
    assert_eq!(&concat_even_fibonaccis(6), "28");
    assert_eq!(&concat_even_fibonaccis(9), "2834");
}
