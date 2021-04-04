use itertools::Itertools;
use std::collections::HashSet;
use std::ops::BitAnd;

fn main() {
    let file = include_str!("../../../inputs/day6");

    let s1 = file
        .lines()
        .group_by(|line| line.is_empty())
        .into_iter()
        .filter(|(cond, _)| !*cond)
        .map(|(_, answers)| answers)
        .map(|answers| {
            answers.into_iter().map(|line| line.chars()).flatten().filter(|c| c.is_alphabetic()).collect::<HashSet<char>>()
        })
        .map(|hs| hs.len())
        .sum::<usize>();

    dbg!(s1);

    let s2 = file
        .lines()
        .group_by(|line| line.is_empty())
        .into_iter()
        .filter(|(cond, _)| !*cond)
        .map(|(_, answers)| answers)
        .map(|answers| {
            answers.into_iter().map(|line| {
                line.chars().filter(|c| c.is_alphabetic()).collect::<HashSet<char>>()
            }).collect::<Vec<_>>()
        })
        .map(|mut answers| {
            let last = answers.pop().unwrap_or_default();

            answers.into_iter().fold(last, |acc, hs| {
                acc.bitand(&hs)
            }).len()
        })
        .sum::<usize>();

    dbg!(s2);
}
