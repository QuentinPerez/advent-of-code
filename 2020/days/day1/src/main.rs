use std::collections::HashSet;
use std::error::Error;

fn resolve(input: &HashSet<i32>, nb_entries: i32, sum: i32) -> Option<i32> {
    if nb_entries == 1 {
        input.get(&sum).copied()
    } else {
        input
            .iter()
            .filter_map(|v| resolve(input, nb_entries - 1, sum - v).map(|res| v * res))
            .next()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../../../inputs/day1")
        .lines()
        .map(str::parse)
        .collect::<Result<HashSet<_>, _>>()?;

    dbg!(resolve(&input, 2, 2020));
    dbg!(resolve(&input, 3, 2020));
    Ok(())
}
