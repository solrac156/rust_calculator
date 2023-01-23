use regex::Regex;

fn math_operation(expression: Regex, mut operation: String, operator: &str) -> String {
    loop {
        let caps = expression.captures(operation.as_str());
        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();
        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value : i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value : i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let mut result : i32 = 0;
        if operator == "+" {
            result = left_value + right_value;
        } else if operator == "-" {
            result = left_value - right_value;
        } else if operator == "*" {
            result = left_value * right_value;
        } else if operator == "/"{
            result = left_value / right_value;
        }
        operation = operation.replace(cap_expression, &result.to_string())
    }
    return operation
}

fn main() {
    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let re_mult = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();
    let re_division = Regex::new(r"(\d+)\s?/\s?(\d+)").unwrap();
    let re_subtract = Regex::new(r"(\d+)\s?-\s?(\d+)").unwrap();
    println!("Please, enter an expression: ");
    let mut expression = String::new();
    std::io::stdin().read_line(&mut expression).unwrap();
    expression = math_operation(re_mult, expression, "*");
    expression = math_operation(re_division, expression, "/");
    expression = math_operation(re_add, expression, "+");
    expression = math_operation(re_subtract, expression, "-");
    println!("Result: {}", expression);
}
