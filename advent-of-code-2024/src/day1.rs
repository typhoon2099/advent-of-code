use std::collections::HashMap;

pub fn part1() -> usize {
    let (list1, list2) = list_data();

    let mut result = 0;
    for (pos, value) in list1.iter().enumerate() {
        let distance: usize = ((*value as isize) - (list2[pos] as isize))
            .abs()
            .try_into()
            .unwrap();

        result += distance;
    }

    result
}

pub fn part2() -> usize {
    let (list1, list2) = list_data();
    let mut totals = HashMap::new();

    for value in list2.iter() {
        totals.entry(value).and_modify(|total| *total += 1).or_insert(1);
    }

    list1
        .iter()
        .map(|value| value * *totals.get(value).unwrap_or(&0) as usize)
        .sum()
}

fn list_data() -> (Vec<usize>, Vec<usize>) {
    let raw = String::from_utf8(include_bytes!("../data/01-1.txt").to_vec()).unwrap();
    let lines: Vec<&str> = raw.lines().collect();

    let mut list1 = vec![];
    let mut list2 = vec![];

    for line in lines.into_iter() {
        let numbers: Vec<usize> = line.split("   ").map(|i| i.parse().unwrap()).collect();

        list1.push(*numbers.first().unwrap());
        list2.push(*numbers.last().unwrap());
    }

    list1.sort();
    list2.sort();

    (list1, list2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 1938424);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 22014209);
    }
}
