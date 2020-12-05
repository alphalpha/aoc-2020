pub fn solve(input: &str) {
    let lines = crate::get_lines(input);
    let ids = get_ids(&lines);
    let r_1 = *ids.iter().max().unwrap();
    let r_2 = my_id(&ids).unwrap();

    assert!(r_1 == 818 && r_2 == 559);
    println!("Day 5. Answer A: {}. Answer B: {}", r_1, r_2);
}

fn my_id(ids: &Vec<usize>) -> Option<usize> {
    let mut prev = ids[0];
    for id in ids.iter().skip(1) {
        if id - prev == 2 {
            return Some(prev + 1);
        }
        prev = *id;
    }
    None
}

fn get_ids(codes: &Vec<String>) -> Vec<usize> {
    let mut ids: Vec<usize> = codes.iter().map(|c| get_id(c)).collect();
    ids.sort();
    ids
}

fn get_id(code: &str) -> usize {
    let code: String = code
        .chars()
        .map(|c| if c == 'B' || c == 'R' { '1' } else { '0' })
        .collect();
    let (row_code, column_code) = code.split_at(7);
    let row = usize::from_str_radix(row_code, 2).unwrap();
    let column = usize::from_str_radix(column_code, 2).unwrap();
    row * 8 + column
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids() {
        let lines = vec![
            "BFFFBBFRRR".to_string(),
            "FFFBBBFRRR".to_string(),
            "BBFFBBFRLL".to_string(),
        ];
        let expected = vec![119, 567, 820];
        assert_eq!(get_ids(&lines), expected);
    }

    #[test]
    fn find_my_id() {
        let ids = vec![5, 6, 8, 12];
        assert_eq!(my_id(&ids), Some(7));
    }
}
