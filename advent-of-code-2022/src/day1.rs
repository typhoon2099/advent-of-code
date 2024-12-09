pub fn part1() -> usize {
    elves()
        .iter()
        .map(|x| x.iter().sum())
        .max()
        .unwrap()
}

pub fn part2() -> usize {
    let mut top_three = elves()
        .iter()
        .map(|x| x.iter().sum())
        .collect::<Vec<usize>>();
    top_three.sort_by(|a, b| b.cmp(a));

    top_three[0..=2].iter().sum()
}

fn elves() -> Vec<Vec<usize>> {
    include_str!("../data/day01.txt")
        .strip_suffix('\n')
        .unwrap()
        .split("\n\n")
        .map(string_to_calories)
        .collect()
}

fn string_to_calories(raw: &str) ->  Vec<usize> {
    raw.lines().map(|x| x.parse::<usize>().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 67016);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 200116);
    }
}
