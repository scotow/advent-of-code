use serde_json::Value;

type Validator = fn(&Value) -> bool;

#[aoc_generator(day12)]
fn input_generator(input: &str) -> Value {
    input.parse().unwrap()
}

#[aoc(day12, part1)]
fn part1(input: &Value) -> i64 {
    extract(input, None)
}

#[aoc(day12, part2)]
fn part2(input: &Value) -> i64 {
    extract(input, Some(has_red))
}

fn extract(input: &Value, validator: Option<Validator>) -> i64 {
    if validator.map(|f| f(input)).unwrap_or(false) { return 0 }
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
        _ => false
    }
}
