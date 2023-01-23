use regex::Regex;

fn main() {
    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let re_mult = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();
    let re_division = Regex::new(r"(\d+)\s?/\s?(\d+)").unwrap();
    let re_subtract = Regex::new(r"(\d+)\s?-\s?(\d+)").unwrap();
    println!("Please, enter an expression: ");
    let mut expression = String::new();
    std::io::stdin().read_line(&mut expression).unwrap();
    loop {
        let caps = re_mult.captures(expression.as_str());
        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();
        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value : i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value : i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let mult = left_value * right_value;
        expression = expression.replace(cap_expression, &mult.to_string())
    }
    loop {
        let caps = re_division.captures(expression.as_str());
        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();
        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value : i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value : i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let division = left_value / right_value;
        expression = expression.replace(cap_expression, &division.to_string())
    }
    loop {
        let caps = re_add.captures(expression.as_str());
        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();
        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value : i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value : i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let addition = left_value + right_value;
        expression = expression.replace(cap_expression, &addition.to_string())
    }
    loop {
        let caps = re_subtract.captures(expression.as_str());
        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();
        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value : i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value : i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let subtraction = left_value - right_value;
        expression = expression.replace(cap_expression, &subtraction.to_string())
    }
    println!("Result: {}", expression);
}
