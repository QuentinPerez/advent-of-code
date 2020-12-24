fn main() {
    let input = include_str!("../../../inputs/day2")
        .lines()
        .map(|l| l.split(' ').into_iter().take(3).collect::<Vec<_>>())
        .map(|parts| {
            let range = {
                let r = parts[0]
                    .split('-')
                    .take(2)
                    .map(str::parse)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();
                r[0]..=r[1]
            };
            let (letter, password) = (parts[1].chars().next().unwrap(), parts[2]);

            (range, letter, password)
        })
        .collect::<Vec<_>>();

    let count_v1 = input
        .iter()
        .filter(|(range, letter, password)| {
            range.contains(&password.chars().filter(|l| l == letter).count())
        })
        .count();

    let count_v2 = input
        .iter()
        .filter(|(range, letter, password)| {
            (password.chars().nth(range.start() - 1).unwrap() == *letter)
                ^ (password.chars().nth(range.end() - 1).unwrap() == *letter)
        })
        .count();

    dbg!(count_v1, count_v2);
}
