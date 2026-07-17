// Exercise 24: JSON From Scratch
//
// Demonstrates: a recursive enum modeling a dynamic value (`Value`), a
// small hand-written recursive-descent parser, and a serializer — the
// shape of what `serde_json` generates for you via derive macros. Written
// by hand here to stay dependency-free; a real project should use serde.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    // BTreeMap keeps keys sorted, which makes serialized output deterministic
    // and this exercise's assertions/printing stable.
    Object(BTreeMap<String, Value>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Bool(b) => write!(f, "{b}"),
            Value::Number(n) => write!(f, "{n}"),
            Value::String(s) => write!(f, "\"{}\"", s.replace('"', "\\\"")),
            Value::Array(items) => {
                write!(f, "[")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{item}")?;
                }
                write!(f, "]")
            }
            Value::Object(map) => {
                write!(f, "{{")?;
                for (i, (k, v)) in map.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "\"{k}\":{v}")?;
                }
                write!(f, "}}")
            }
        }
    }
}

struct Parser<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.chars.peek(), Some(c) if c.is_whitespace()) {
            self.chars.next();
        }
    }

    fn parse_value(&mut self) -> Result<Value, String> {
        self.skip_whitespace();
        match self.chars.peek() {
            Some('n') => self.parse_literal("null", Value::Null),
            Some('t') => self.parse_literal("true", Value::Bool(true)),
            Some('f') => self.parse_literal("false", Value::Bool(false)),
            Some('"') => self.parse_string().map(Value::String),
            Some('[') => self.parse_array(),
            Some('{') => self.parse_object(),
            Some(c) if c.is_ascii_digit() || *c == '-' => self.parse_number(),
            other => Err(format!("unexpected character: {other:?}")),
        }
    }

    fn parse_literal(&mut self, literal: &str, value: Value) -> Result<Value, String> {
        for expected in literal.chars() {
            match self.chars.next() {
                Some(c) if c == expected => {}
                other => return Err(format!("expected '{literal}', got {other:?}")),
            }
        }
        Ok(value)
    }

    fn parse_string(&mut self) -> Result<String, String> {
        self.chars.next(); // consume opening quote
        let mut s = String::new();
        loop {
            match self.chars.next() {
                Some('"') => return Ok(s),
                Some('\\') => match self.chars.next() {
                    Some('"') => s.push('"'),
                    Some('\\') => s.push('\\'),
                    Some('n') => s.push('\n'),
                    other => return Err(format!("unsupported escape: {other:?}")),
                },
                Some(c) => s.push(c),
                None => return Err("unterminated string".to_string()),
            }
        }
    }

    fn parse_number(&mut self) -> Result<Value, String> {
        let mut raw = String::new();
        while matches!(self.chars.peek(), Some(c) if c.is_ascii_digit() || matches!(c, '-' | '+' | '.' | 'e' | 'E'))
        {
            raw.push(self.chars.next().unwrap());
        }
        raw.parse::<f64>()
            .map(Value::Number)
            .map_err(|e| format!("invalid number '{raw}': {e}"))
    }

    fn parse_array(&mut self) -> Result<Value, String> {
        self.chars.next(); // consume '['
        let mut items = Vec::new();
        self.skip_whitespace();
        if self.chars.peek() == Some(&']') {
            self.chars.next();
            return Ok(Value::Array(items));
        }
        loop {
            items.push(self.parse_value()?);
            self.skip_whitespace();
            match self.chars.next() {
                Some(',') => continue,
                Some(']') => return Ok(Value::Array(items)),
                other => return Err(format!("expected ',' or ']', got {other:?}")),
            }
        }
    }

    fn parse_object(&mut self) -> Result<Value, String> {
        self.chars.next(); // consume '{'
        let mut map = BTreeMap::new();
        self.skip_whitespace();
        if self.chars.peek() == Some(&'}') {
            self.chars.next();
            return Ok(Value::Object(map));
        }
        loop {
            self.skip_whitespace();
            let key = self.parse_string()?;
            self.skip_whitespace();
            match self.chars.next() {
                Some(':') => {}
                other => return Err(format!("expected ':', got {other:?}")),
            }
            let value = self.parse_value()?;
            map.insert(key, value);
            self.skip_whitespace();
            match self.chars.next() {
                Some(',') => continue,
                Some('}') => return Ok(Value::Object(map)),
                other => return Err(format!("expected ',' or '}}', got {other:?}")),
            }
        }
    }
}

fn parse(input: &str) -> Result<Value, String> {
    Parser::new(input).parse_value()
}

fn main() {
    println!("=== Exercise 24: JSON From Scratch ===");

    // Section 1: parsing scalars
    println!("\n--- Section 1: scalars ---");
    for input in ["null", "true", "false", "42", "-3.5"] {
        println!("{input:?} -> {:?}", parse(input));
    }

    // Section 2: parsing a nested structure
    println!("\n--- Section 2: nested object/array ---");
    let input = r#"{"name":"Ferris","age":10,"languages":["rust","c","c++"],"active":true}"#;
    let value = parse(input).unwrap();
    println!("parsed: {value:?}");

    // Section 3: round-tripping through Display back to a JSON string
    println!("\n--- Section 3: serialize back to text ---");
    println!("serialized: {value}");

    // Section 4: navigating the parsed value
    println!("\n--- Section 4: reading fields ---");
    if let Value::Object(map) = &value {
        if let Some(Value::String(name)) = map.get("name") {
            println!("name field: {name}");
        }
        if let Some(Value::Array(langs)) = map.get("languages") {
            println!("language count: {}", langs.len());
        }
    }

    // Section 5: a parse error
    println!("\n--- Section 5: error handling ---");
    println!("{:?}", parse("{not valid json"));

    println!("\nNotes:");
    println!("  - This ~150-line parser is what serde_json + #[derive(Serialize, Deserialize)] replace in practice.");
    println!("  - Value is a recursive enum — Array/Object hold more Values, just like exercise 16's Box<List>.");
    println!("  - BTreeMap (not HashMap) keeps object keys sorted so serialized output is deterministic.");
    println!("  - A real project should use serde: far more correct (full Unicode escapes, number precision, etc).");
}
