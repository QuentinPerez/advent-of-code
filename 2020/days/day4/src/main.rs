use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let file = include_str!("../../../inputs/day4");

    let identities = file
        .lines()
        .group_by(|line| line.is_empty())
        .into_iter()
        .filter(|(cond, _)| !*cond)
        .map(|(_, identity)| identity)
        .map(|identity| {
            identity
                .into_iter()
                .map(|l| l.split(' '))
                .flatten()
                .filter_map(|v| v.split_once(':'))
                .map(|(k, v)| {
                    let valid = match k {
                        "byr" => (1920..=2002).contains(&v.parse::<i32>().unwrap_or(0)),
                        "iyr" => (2010..=2020).contains(&v.parse::<i32>().unwrap_or(0)),
                        "eyr" => (2020..=2030).contains(&v.parse::<i32>().unwrap_or(0)),
                        "hgt" => match v.split_at(v.len() - 2) {
                            (v, "cm") => (150..=193).contains(&v.parse::<i32>().unwrap_or(0)),
                            (v, "in") => (59..=76).contains(&v.parse::<i32>().unwrap_or(0)),
                            _ => false,
                        },
                        "hcl" => {
                            v.chars()
                                .enumerate()
                                .map(|(i, c)| {
                                    if i == 0 && c == '#' {
                                        true
                                    } else {
                                        matches!(c, 'a'..='f' | '0'..='9')
                                    }
                                })
                                .filter(|&v| v)
                                .count()
                                == 7
                        }
                        "ecl" => {
                            matches!(v, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
                        }
                        "pid" => v.chars().map(|c| c.is_numeric()).filter(|&v| v).count() == 9,
                        _ => false,
                    };

                    (k, valid)
                })
                .collect::<HashMap<_, _>>()
        })
        .collect::<Vec<_>>();

    let s1 = identities
        .iter()
        .map(|identity| {
            ["ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"]
                .iter()
                .filter_map(|req| identity.get(req))
                .count()
        })
        .filter(|&req| req == 7)
        .count();
    dbg!(s1);

    let s2 = identities
        .iter()
        .map(|identity| {
            ["ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"]
                .iter()
                .filter_map(|req| identity.get(req))
                .filter(|&&valid| valid)
                .count()
        })
        .filter(|&req| req == 7)
        .count();

    dbg!(s2);
}
