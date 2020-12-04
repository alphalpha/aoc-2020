use regex::Regex;

struct Password {
    policy: (i32, i32),
    character: char,
    sequence: String,
}

pub fn solve(input: &str) {
    let lines = crate::get_lines(input);
    let pws = passwords(&lines);
    let vpws_1 = valid_passwords_1(&pws);
    let vpws_2 = valid_passwords_2(&pws);
    assert!(vpws_1 == 483 && vpws_2 == 482);
    println!("Day 2. Answer A: {}. Answer B: {}", vpws_1, vpws_2);
}

fn valid_passwords_2(pws: &Vec<Password>) -> i32 {
    let mut valid_num = 0;
    for pw in pws {
        let one = pw
            .sequence
            .chars()
            .nth(pw.policy.0 as usize - 1)
            .map(|x| x == pw.character)
            .unwrap();
        let two = pw
            .sequence
            .chars()
            .nth(pw.policy.1 as usize - 1)
            .map(|x| x == pw.character)
            .unwrap();
        if (one || two) && (one != two) {
            valid_num += 1;
        }
    }
    valid_num
}

fn valid_passwords_1(pws: &Vec<Password>) -> i32 {
    let mut valid_num = 0;
    for pw in pws {
        let count = pw.sequence.matches(pw.character).count() as i32;
        if count >= pw.policy.0 && count <= pw.policy.1 {
            valid_num += 1;
        }
    }
    valid_num
}

fn passwords(lines: &Vec<String>) -> Vec<Password> {
    let re = Regex::new(r"(\d*)-(\d*) (\w): (\w*)").unwrap();
    lines
        .iter()
        .map(|s| {
            let caps = re.captures(s).unwrap();
            let p1 = caps[1].parse::<i32>().unwrap();
            let p2 = caps[2].parse::<i32>().unwrap();
            let character = caps[3].parse::<char>().unwrap();
            let sequence = caps[4].to_string();
            Password {
                policy: (p1, p2),
                character,
                sequence,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_1() {
        let input = vec![
            "1-3 a: abcde".to_string(),
            "1-3 b: cdefg".to_string(),
            "2-9 c: ccccccccc".to_string(),
        ];
        let pws = passwords(&input);
        assert_eq!(valid_passwords_1(&pws), 2);
    }

    #[test]
    fn task_2() {
        let input = vec![
            "1-3 a: abcde".to_string(),
            "1-3 b: cdefg".to_string(),
            "2-9 c: ccccccccc".to_string(),
        ];
        let pws = passwords(&input);
        assert_eq!(valid_passwords_2(&pws), 1);
    }
}
