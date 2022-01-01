use std::cmp::Ordering;

type AnyError = Box<dyn std::error::Error>;

const INPUT: &str = include_str!("../../../inputs/day3");

fn main() -> Result<(), AnyError> {
    let entries = || {
        std::iter::repeat(()).scan(0, |index, _| {
            let mut values = INPUT
                .lines()
                .map(|line| line.chars())
                .flat_map({
                    let index = *index;

                    move |mut binary| binary.nth(index).map(|value| value == '1')
                })
                .peekable();

            *index += 1;
            values.peek()?;
            Some(values)
        })
    };

    let (gamma, epsilon) = {
        let gamma = entries()
            .into_iter()
            .enumerate()
            .fold(0, |acc, (index, values)| {
                let (zeros, ones) = values.fold((0, 0), |(zeros, ones), value| {
                    ((zeros + (!value as u32)), (ones + (value as u32)))
                });

                acc | (((zeros > ones) as u32) << (11 - index))
            });

        (gamma, (!gamma & ((1 << 12) - 1)))
    };
    dbg!(gamma * epsilon);

    let oxy = compute(entries(), |ones, zeros| {
        match ones.len().cmp(&zeros.len()) {
            Ordering::Less => zeros,
            Ordering::Equal | Ordering::Greater => ones,
        }
    });
    let coo = compute(entries(), |ones, zeros| {
        match zeros.len().cmp(&ones.len()) {
            Ordering::Less | Ordering::Equal => zeros,
            Ordering::Greater => ones,
        }
    });

    dbg!(oxy * coo);

    Ok(())
}

fn compute<T>(
    binaries: impl IntoIterator<Item = T> + Clone,
    f: impl Fn(Vec<usize>, Vec<usize>) -> Vec<usize>,
) -> usize
where
    T: Iterator<Item = bool>,
{
    let mut retain: Option<Vec<usize>> = None;

    for binary in binaries.clone() {
        let (zeros, ones) =
            binary
                .enumerate()
                .fold((vec![], vec![]), |(mut zeros, mut ones), (index, value)| {
                    if let Some(retain) = &retain {
                        if !retain.contains(&index) {
                            return (zeros, ones);
                        }
                    }

                    if value {
                        ones.push(index);
                    } else {
                        zeros.push(index);
                    }

                    (zeros, ones)
                });

        let r = f(ones, zeros);
        if r.len() == 1 {
            let index = *r.get(0).expect("checked above");

            return binaries
                .into_iter()
                .map(|binary| binary.into_iter().nth(index).expect("must exist"))
                .fold(0, |mut acc, value| {
                    acc <<= 1;
                    acc | value as usize
                });
        }
        retain = Some(r)
    }

    unreachable!()
}
