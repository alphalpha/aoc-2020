pub fn solve(input: &str) {
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let lines = crate::get_lines(input);
    let r_1 = num_of_trees(&lines, slopes[1]);
    let r_2 = slopes
        .iter()
        .map(|&slope| num_of_trees(&lines, slope))
        .fold(1, |acc, x| acc * x);
    assert!(r_1 == 274 && r_2 == 6050183040);
    println!("Day 3. Answer A: {}. Answer B: {}", r_1, r_2);
}

fn num_of_trees(tree_map: &Vec<String>, slope: (usize, usize)) -> usize {
    let mut count = 0;
    let mut x = 1 + slope.0;
    for line in tree_map
        .iter()
        .skip(1)
        .enumerate()
        .filter(|&(i, _)| (i + 1) % slope.1 == 0)
        .map(|(_, v)| v)
    {
        if line.chars().cycle().nth(x - 1) == Some('#') {
            count += 1;
        }
        x += slope.0;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const TREE_MAP: &'static [&'static str] = &[
        "..##.......",
        "#...#...#..",
        ".#....#..#.",
        "..#.#...#.#",
        ".#...##..#.",
        "..#.##.....",
        ".#.#.#....#",
        ".#........#",
        "#.##...#...",
        "#...##....#",
        ".#..#...#.#",
    ];
    #[test]
    fn task_1() {
        let slope = (3, 1);
        assert_eq!(
            num_of_trees(&TREE_MAP.iter().map(|s| s.to_string()).collect(), slope),
            7
        );
    }

    #[test]
    fn task_2() {
        let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        let tree_map = TREE_MAP.iter().map(|s| s.to_string()).collect();
        let result = slopes
            .iter()
            .map(|&slope| num_of_trees(&tree_map, slope))
            .fold(1, |acc, x| acc * x);
        assert_eq!(result, 336);
    }
}
