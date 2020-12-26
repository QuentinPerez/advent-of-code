use std::collections::HashMap;

fn main() {
    let file = include_str!("../../../inputs/day4");

    let identities = file
        .lines()
        .hold_until(|s| s.is_empty())
        .map(|identity| {
            identity
                .into_iter()
                .map(|l| l.split(' '))
                .flatten()
                .map(|v| {
                    let mut split = v.split(':');
                    (split.next().unwrap(), split.next().unwrap())
                })
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

// -----------------------------------------------------------------------------

struct HoldUntil<I, T, F> {
    iter: I,
    f: F,
    hold_until: Vec<T>,
}

impl<I, F> Iterator for HoldUntil<I, I::Item, F>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next().map(|v| {
                let cond = (self.f)(&v);
                (v, cond)
            }) {
                Some((_, cond)) if cond => {
                    return Some(self.hold_until.split_off(0));
                }
                Some((v, cond)) if !cond => self.hold_until.push(v),
                None => {
                    let vec = self.hold_until.split_off(0);

                    if !vec.is_empty() {
                        return Some(vec);
                    } else {
                        return None;
                    }
                }
                Some(_) => unreachable!(), // ???
            }
        }
    }
}

trait HoldUntilIterator<T> {
    fn hold_until<F>(self, f: F) -> HoldUntil<Self, T, F>
    where
        F: Fn(&T) -> bool,
        Self: Sized;
}

impl<I, T> HoldUntilIterator<T> for I
where
    I: Iterator<Item = T>,
{
    fn hold_until<F>(self, f: F) -> HoldUntil<Self, T, F>
    where
        F: Fn(&T) -> bool,
        Self: Sized,
    {
        HoldUntil {
            iter: self,
            f,
            hold_until: vec![],
        }
    }
}
