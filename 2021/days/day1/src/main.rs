#![feature(array_windows)]

type AnyError = Box<dyn std::error::Error>;

fn main() -> Result<(), AnyError> {
    let input = include_str!("../../../inputs/day1")
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<u32>, _>>()?;

    let count = input.array_windows().filter(|[r, l]| r < l).count();

    dbg!(count);

    let input = input
        .array_windows()
        .map(|arr: &[u32; 3]| arr.iter().sum())
        .collect::<Vec<u32>>();

    let count = input.array_windows().filter(|[r, l]| r < l).count();

    dbg!(count);
    Ok(())
}
