use std::collections::HashSet;

pub fn part1() -> usize {
    compartments()
        .iter()
        .flat_map(|compartment| {
            compartment.0.intersection(&compartment.1).collect::<Vec<_>>()
        })
        .map(|&x| x as usize)
        .sum::<usize>()
}

pub fn part2() -> usize {
    4
}

fn compartments() -> Vec<(HashSet<u8>, HashSet<u8>)> {
    let data = include_str!("../data/day03.txt");

    data
        .lines()
        .into_iter()
        .map(rucksack_compartments)
        .collect()
}

fn rucksack_compartments<'a>(rucksack: &'a str) -> (HashSet<u8>, HashSet<u8>) {
    let mid_point = rucksack.len() / 2;
    (compartment(&rucksack[..mid_point]), compartment(&rucksack[mid_point..]))
}

fn compartment(raw: &str) -> HashSet<u8> {
    let mut output = HashSet::new();

    for char in raw.chars() {
        output.insert(char_to_score(char));
    }

    output
}

fn char_to_score(c: char) -> u8 {
    if (c as u8) > 96 {
        return (c as u8) - 96;
    } else {
        return (c as u8) - 38;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 8394);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(), 12683);
    // }
}
