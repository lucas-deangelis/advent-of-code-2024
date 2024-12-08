use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input file");

    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut res = mul_regex.captures_iter(&input).fold(0, |acc, caps| acc + parse_caps(&caps[1], &caps[2]));

    println!("Silver star: {}", res);

    let input = std::fs::read_to_string("input.txt").expect("Failed to read input file");
    // let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string();
    let dopped_input = "do()".to_string() + &input.to_string() + "don't()";

    let re = Regex::new(r"(do\(\)|don't\(\))([^d]*)").unwrap();

    let mut res2 = 0;

    for el in re.captures_iter(&dopped_input) {
        if &el[1] == "do()" {
            let a = mul_regex.captures_iter(&el[1]).fold(0, |acc, caps| acc + parse_caps(&caps[1], &caps[2]));
            res2 += a;
        }
    }

    println!("Gold star: {}", res2);
}

fn parse_caps(a: &str, b: &str) -> i32 {
    let x = a.parse::<i32>().unwrap();
    let y = b.parse::<i32>().unwrap();
    x * y
}