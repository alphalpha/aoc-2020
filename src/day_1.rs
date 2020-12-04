pub fn solve(input: &str) {
    let lines = crate::get_lines(input);
    let nums = numbers(&lines);
    let r_1 = two(&nums).unwrap();
    let r_2 = three(&nums).unwrap();
    assert!(r_1 == 41979 && r_2 == 193416912);
    println!("Day 1. Answer A: {}. Answer B: {}", r_1, r_2);
}

pub fn two(numbers: &Vec<i32>) -> Option<i32> {
    for num_1 in numbers {
        for num_2 in numbers {
            if num_1 != num_2 && (num_1 + num_2) == 2020 {
                return Some(num_1 * num_2);
            }
        }
    }
    None
}

pub fn three(numbers: &Vec<i32>) -> Option<i32> {
    for num_1 in numbers {
        for num_2 in numbers {
            for num_3 in numbers {
                if num_1 != num_2
                    && num_1 != num_3
                    && num_2 != num_3
                    && (num_1 + num_2 + num_3) == 2020
                {
                    return Some(num_1 * num_2 * num_3);
                }
            }
        }
    }
    None
}

pub fn numbers(lines: &Vec<String>) -> Vec<i32> {
    let mut numbers: Vec<i32> = lines.iter().map(|s| s.parse::<i32>().unwrap()).collect();
    numbers.sort();
    numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_1() {
        let expenses = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(two(&expenses), Some(514579));
    }

    #[test]
    fn task_2() {
        let expenses = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(three(&expenses), Some(241861950));
    }
}
