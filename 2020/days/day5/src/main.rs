#![feature(map_first_last)]

use std::ops::Range;

fn main() {
    let res = {
        let mut res = include_bytes!("../../../inputs/day5")
            .chunks(11)
            .map(|line| line[..10].split_at(7))
            .map(|(r, l)| {
                r.iter().fold(0..127, split_range).start * 8
                    + l.iter().fold(0..7, split_range).start
            })
            .collect::<Vec<_>>();
        res.sort_unstable();
        res
    };

    let s1 = res.last();
    dbg!(s1);

    let s2 = res.windows(2).find(|v| v[0] == v[1] - 2).unwrap()[0] + 1;
    dbg!(s2);
}

fn split_range(range: Range<i32>, v: &u8) -> Range<i32> {
    const BACK: u8 = b'B';
    const FRONT: u8 = b'F';

    const RIGHT: u8 = b'R';
    const LEFT: u8 = b'L';

    let delta = (range.end - range.start) / 2;
    match *v {
        FRONT | LEFT => range.start..(range.start + delta),
        BACK | RIGHT => (range.start + delta + 1)..range.end,
        _ => unreachable!(),
    }
}
