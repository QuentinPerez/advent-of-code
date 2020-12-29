#![feature(map_first_last)]

use std::ops::Range;

fn main() {
    let res = {
        let mut res = include_str!("../../../inputs/day5")
            .lines()
            .map(str::as_bytes)
            .map(|chars| chars.split_at(7))
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
    const BACK: u8 = 'B' as u8;
    const FRONT: u8 = 'F' as u8;

    const RIGHT: u8 = 'R' as u8;
    const LEFT: u8 = 'L' as u8;

    let delta = (range.end - range.start) / 2;
    match *v {
        FRONT | LEFT => range.start..(range.start + delta),
        BACK | RIGHT => (range.start + delta + 1)..range.end,
        _ => unreachable!(),
    }
}
