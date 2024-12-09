pub fn part1() -> usize {
    depths()
        .windows(2)
        .filter(|window| window[1] > window[0])
        .count()
}

pub fn part2() -> usize {
    depths()
        .windows(3)
        .map(|window| window[0] + window[1] + window[2])
        .collect::<Vec<usize>>()
        .windows(2)
        .filter(|window| window[1] > window[0] )
        .count()
}

fn depths() -> Vec<usize> {
    let raw_depths = String::from_utf8(include_bytes!("../data/day1.txt").to_vec()).unwrap();
    let depth_strings: Vec<&str> = raw_depths.split_whitespace().collect();

    depth_strings
        .iter()
        .map(|x| x.parse::<usize>().unwrap_or(0))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 1616);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1645);
    }
}
