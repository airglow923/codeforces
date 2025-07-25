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
    let [n_towers, tower_init] = <[usize; 2]>::try_from(take_vector()).unwrap();
    let mut towers = take_vector();
    let h_init = towers[tower_init - 1];

    towers.sort_unstable();

    let h_highest = towers.last().unwrap();

    if h_init == *h_highest {
        println!("yes");
        return;
    }

    let mut i_high = 0;

    for (i, tower) in towers.iter().enumerate() {
        if (h_init < *tower) {
            i_high = i;
            break;
        }
    }

    let high_towers = &towers[i_high..];
    let mut h_current = h_init;

    for tower in high_towers {
        if *tower <= h_current {
            continue;
        }

        if h_current + h_init < *tower {
            println!("no");
            return;
        } else {
            if *tower == *h_highest {
                println!("yes");
                return;
            }

            h_current = *tower;
        }
    }

    println!("no");
}

pub fn solve() {
    let n_test_cases = take_int();

    for _ in 0..n_test_cases {
        solve_impl();
    }
}
