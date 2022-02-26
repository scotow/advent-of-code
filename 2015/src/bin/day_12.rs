advent_of_code_2015::main!();

use serde_json::Value;

type Validator = fn(&Value) -> bool;

fn generator(input: &str) -> Value {
    input.parse().unwrap()
}

fn part_1(input: Value) -> i64 {
    extract(&input, None)
}

fn part_2(input: Value) -> i64 {
    extract(&input, Some(has_red))
}

fn extract(input: &Value, validator: Option<Validator>) -> i64 {
    if validator.map(|f| f(input)).unwrap_or(false) {
        return 0;
    }
    match input {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(a) => a.iter().map(|v| extract(v, validator)).sum(),
        Value::Object(o) => o.values().map(|v| extract(v, validator)).sum(),
        _ => 0,
    }
}

fn has_red(input: &Value) -> bool {
    match input {
        Value::Object(o) => o.values().any(|v| v.as_str() == Some("red")),
        _ => false,
    }
}
