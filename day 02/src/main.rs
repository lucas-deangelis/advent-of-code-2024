fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input file");
    let safe_levels_count = input.lines().map(|line| is_safe(line)).filter(|el| *el == true).count();

    let input = std::fs::read_to_string("input.txt").expect("Failed to read input file");
    let safe_levels_count_gold = input.lines().map(|line| is_safe_multiple(line)).filter(|el| *el == true).count();

    println!("Silver star: {}", safe_levels_count);
    println!("Gold star: {}", safe_levels_count_gold);
}

fn levels_string_to_numbers(levels: &str) -> Vec<i32> {
    levels.split_whitespace().map(|el| str::parse::<i32>(el).unwrap()).collect::<Vec<i32>>()
}

fn is_safe(levels: &str) -> bool {
    return is_safe_single(levels_string_to_numbers(levels));
}

fn is_safe_single(v: Vec<i32>) -> bool {
    let increasing = v[0] < v[1];
    let mut current = &v[0];

    for val in &v[1..v.len()] {
        if val > current && !increasing {
            return false;
        }

        if val < current && increasing {
            return false;
        }

        if val == current || (current - val).abs() > 3 { 
            return false
        }

        current = val;
    }

    return true;
}

fn is_safe_multiple(levels: &str) -> bool {
    let v = levels_string_to_numbers(levels);

    if is_safe_single(v.clone()) {
        return true;
    }

    return vectors_without_one(v).iter().map(|vec| is_safe_single(vec.to_vec())).any(|f| f == true)
}

fn vectors_without_one(vec: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();


    for i in 0..vec.len() {
        let mut new = vec.clone();
        new.remove(i);
        res.push(new);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_silver() {
        let test_cases = vec![
            ("7 6 4 2 1", true),
            ("1 2 7 8 9", false),
            ("9 7 6 2 1", false),
            ("1 3 2 4 5", false),
            ("8 6 4 4 1", false),
            ("1 3 6 7 9", true),
        ];

        for (input, expected) in test_cases {
            assert_eq!(is_safe(input), expected, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_safe_gold() {
        let test_cases = vec![
            ("7 6 4 2 1", true),
            ("1 2 7 8 9", false),
            ("9 7 6 2 1", false),
            ("1 3 2 4 5", true),
            ("8 6 4 4 1", true),
            ("1 3 6 7 9", true),
        ];

        for (input, expected) in test_cases {
            assert_eq!(is_safe_multiple(input), expected, "Failed for input: {}\ngot: {} expected: {}", input, is_safe_multiple(input), expected);
        }
    }
}
