fn main() {
    let file = include_str!("../../../inputs/day3");

    let input = file
        .lines()
        .map(str::chars)
        .map(|iter| iter.map(|c| c == '#').collect())
        .collect::<Vec<_>>();

    let s1 = count_trees(&input, (3, 1));
    dbg!(s1);

    let s2 = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|dir| count_trees(&input, *dir))
        .product::<usize>();
    dbg!(s2);
}

fn count_trees(input: &[Vec<bool>], dir: (usize, usize)) -> usize {
    let width = input[0].len();

    let mut pos = (0, 0);
    std::iter::repeat(())
        .map(|_| {
            pos = ((pos.0 + dir.0) % width, pos.1 + dir.1);
            pos
        })
        .map(|pos| {
            if pos.1 < input.len() {
                Some(input[pos.1][pos.0])
            } else {
                None
            }
        })
        .take_while(Option::is_some)
        .filter(|v| v.expect("filtered out"))
        .count()
}
