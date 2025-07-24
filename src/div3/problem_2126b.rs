#![allow(unused)]

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
    let [n_days, n_hike] = <[usize; 2]>::try_from(take_vector()).unwrap();
    let rains = take_vector();

    let mut n_hiked = 0;
    let mut days_hiked = 0;
    let mut hiked = false;

    for rain in rains {
        match rain {
            0 => {
                if hiked {
                    hiked = false;
                    continue;
                }

                days_hiked += 1;

                if days_hiked == n_hike {
                    n_hiked += 1;
                    days_hiked = 0;
                    hiked = true;
                }
            }
            1 => {
                hiked = false;
                days_hiked = 0;
            }
            _ => break,
        }
    }

    println!("{n_hiked}");
}

pub fn solve() {
    let n_test_cases = take_int();

    for _ in 0..n_test_cases {
        solve_impl();
    }
}
