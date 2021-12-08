use serde::Deserialize;

type AnyError = Box<dyn std::error::Error>;

const INPUT: &str = include_str!("../../../inputs/day2");

#[derive(Deserialize)]
enum Command {
    #[serde(rename = "up")]
    Up,
    #[serde(rename = "down")]
    Down,
    #[serde(rename = "forward")]
    Forward,
}

fn main() -> Result<(), AnyError> {
    let commands = INPUT
        .lines()
        .map(|line| serde_scan::scan!("{} {}" <- line))
        .collect::<Result<Vec<(Command, u64)>, _>>()?;

    let (h, d) = commands
        .iter()
        .fold((0, 0), |(h, d), (cmd, value)| match cmd {
            Command::Up => (h, d - value),
            Command::Down => (h, d + value),
            Command::Forward => (h + value, d),
        });

    dbg!(h * d);

    let (h, d, _) = commands
        .iter()
        .fold((0, 0, 0), |(h, d, a), (cmd, value)| match cmd {
            Command::Up => (h, d, a - value),
            Command::Down => (h, d, a + value),
            Command::Forward => (h + value, d + value * a, a),
        });

    dbg!(h * d);

    Ok(())
}
