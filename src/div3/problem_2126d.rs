#![allow(unused)]

use std::cmp::Ordering;
use std::convert::TryFrom;
use std::io::stdin;
#[cfg(feature = "not_codeforces")]
use tracing::debug;

fn take_int() -> usize {
    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();

    return input.trim().parse().unwrap();
}

fn take_vector() -> Vec<usize> {
    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();

    let arr: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    return arr;
}

fn take_string() -> Vec<char> {
    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();

    let vec: Vec<char> = input.trim().chars().collect();

    return vec;
}

fn to_string(vec: Vec<char>) -> String {
    return vec.iter().collect::<String>();
}

fn solve_impl() {
    let [n_casinos, n_coins] = <[usize; 2]>::try_from(take_vector()).unwrap();
    let mut data_casino = Vec::new();

    #[cfg(feature = "not_codeforces")]
    tracing::debug!("n_casinos: {n_casinos}, n_coins: {n_coins}");

    for _ in 0..n_casinos {
        let [l, r, real] = <[usize; 3]>::try_from(take_vector()).unwrap();
        #[cfg(feature = "not_codeforces")]
        tracing::debug!("l: {l}, r: {r}, real: {real}");

        if real < n_coins {
            continue;
        }

        data_casino.push((l, r, real));
    }

    data_casino.sort_unstable_by(|a, b| match a.0.cmp(&b.0) {
        Ordering::Equal => a.2.cmp(&b.2),
        v => v,
    });

    #[cfg(feature = "not_codeforces")]
    tracing::debug!("{:?}", data_casino);

    let mut total_coins = n_coins;

    for datum in data_casino {
        if datum.0 > total_coins {
            break;
        }

        if datum.2 > total_coins {
            total_coins = datum.2;
        }
    }

    println!("{total_coins}");
}

pub fn solve() {
    let n_test_cases = take_int();

    for _ in 0..n_test_cases {
        solve_impl();
    }
}
