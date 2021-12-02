type AnyError = Box<dyn std::error::Error>;

fn main() -> Result<(), AnyError> {
    let input = include_str!("../../../inputs/day1")
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<u32>, _>>()?;

    let count = input
        .windows(2)
        .filter(|values| values[0] < values[1])
        .count();

    dbg!(count);

    let input = input
        .windows(3)
        .map(|values| values.iter().sum())
        .collect::<Vec<u32>>();

    let count = input
        .windows(2)
        .filter(|values| values[0] < values[1])
        .count();

    dbg!(count);
    Ok(())
}
