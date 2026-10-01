// Exercise 24: JSON From Scratch
//
// Demonstrates: a recursive enum modeling a dynamic value (`Value`), a
// small hand-written recursive-descent parser, and a serializer — the
// shape of what `serde_json` generates for you via derive macros. Written
// by hand here to stay dependency-free; a real project should use serde.
//
// The parser follows the JSON grammar (RFC 8259) strictly: it rejects
// trailing input, leading zeros, raw control characters in strings, and
// unknown escapes, decodes every escape including `\uXXXX` surrogate pairs,
// and caps nesting depth so hostile input can't overflow the stack.

use std::collections::BTreeMap;
use std::fmt;

// Each `[` or `{` recurses one level. Without a cap, an input of 200,000 `[`
// characters overflows the thread's stack and aborts the whole process —
// a classic denial-of-service bug in hand-written recursive parsers.
const MAX_DEPTH: usize = 128;

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

// ---------------------------------------------------------------------------
// Serialization
// ---------------------------------------------------------------------------

// ONE escaping helper, shared by string values AND object keys — keys are
// JSON strings too, and forgetting to escape them is a classic serializer bug.
fn write_json_string(f: &mut fmt::Formatter<'_>, s: &str) -> fmt::Result {
    write!(f, "\"")?;
    for c in s.chars() {
        match c {
            '"' => write!(f, "\\\"")?,
            '\\' => write!(f, "\\\\")?,
            '\n' => write!(f, "\\n")?,
            '\r' => write!(f, "\\r")?,
            '\t' => write!(f, "\\t")?,
            '\u{08}' => write!(f, "\\b")?,
            '\u{0C}' => write!(f, "\\f")?,
            // Every other control character (U+0000..U+001F) MUST be escaped.
            c if c < '\u{20}' => write!(f, "\\u{:04x}", c as u32)?,
            c => write!(f, "{c}")?,
        }
    }
    write!(f, "\"")
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Bool(b) => write!(f, "{b}"),
            // JSON has no NaN or Infinity. Writing Rust's "NaN"/"inf" would
            // produce invalid JSON, so non-finite numbers become `null` — the
            // same choice JavaScript's JSON.stringify makes.
            Value::Number(n) if !n.is_finite() => write!(f, "null"),
            // f64's Display never uses exponent notation and prints integral
            // values without a fraction (42.0 -> "42"), both valid JSON.
            Value::Number(n) => write!(f, "{n}"),
            Value::String(s) => write_json_string(f, s),
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
                    write_json_string(f, k)?;
                    write!(f, ":{v}")?;
                }
                write!(f, "}}")
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Parsing
// ---------------------------------------------------------------------------

struct Parser<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    depth: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
            depth: 0,
        }
    }

    // JSON whitespace is exactly these four characters — not every Unicode
    // space that char::is_whitespace accepts.
    fn skip_whitespace(&mut self) {
        while matches!(self.chars.peek(), Some(' ' | '\t' | '\n' | '\r')) {
            self.chars.next();
        }
    }

    // Consume `expected` or report what was found instead.
    fn expect(&mut self, expected: char) -> Result<(), String> {
        match self.chars.next() {
            Some(c) if c == expected => Ok(()),
            other => Err(format!("expected {expected:?}, got {other:?}")),
        }
    }

    fn parse_value(&mut self) -> Result<Value, String> {
        self.skip_whitespace();
        match self.chars.peek() {
            Some('n') => self.parse_literal("null", Value::Null),
            Some('t') => self.parse_literal("true", Value::Bool(true)),
            Some('f') => self.parse_literal("false", Value::Bool(false)),
            Some('"') => self.parse_string().map(Value::String),
            Some('[') => self.nested(Self::parse_array),
            Some('{') => self.nested(Self::parse_object),
            Some(c) if c.is_ascii_digit() || *c == '-' => self.parse_number(),
            other => Err(format!("unexpected character: {other:?}")),
        }
    }

    // Wraps one level of array/object recursion with the depth check.
    fn nested(&mut self, parse: fn(&mut Self) -> Result<Value, String>) -> Result<Value, String> {
        self.depth += 1;
        if self.depth > MAX_DEPTH {
            return Err(format!("nesting deeper than {MAX_DEPTH} levels"));
        }
        let result = parse(self);
        self.depth -= 1;
        result
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
        self.expect('"')?; // check the quote, don't just assume it's there
        let mut s = String::new();
        loop {
            match self.chars.next() {
                Some('"') => return Ok(s),
                Some('\\') => s.push(self.parse_escape()?),
                // Raw control characters (including a literal newline) are not
                // allowed inside a JSON string; they must be escaped.
                Some(c) if c < '\u{20}' => {
                    return Err(format!("raw control character {c:?} in string"))
                }
                Some(c) => s.push(c),
                None => return Err("unterminated string".to_string()),
            }
        }
    }

    // Called just after a backslash.
    fn parse_escape(&mut self) -> Result<char, String> {
        match self.chars.next() {
            Some('"') => Ok('"'),
            Some('\\') => Ok('\\'),
            Some('/') => Ok('/'),
            Some('b') => Ok('\u{08}'),
            Some('f') => Ok('\u{0C}'),
            Some('n') => Ok('\n'),
            Some('r') => Ok('\r'),
            Some('t') => Ok('\t'),
            Some('u') => self.parse_unicode_escape(),
            other => Err(format!("invalid escape: {other:?}")),
        }
    }

    // `\uXXXX` encodes a UTF-16 code unit. Characters outside the Basic
    // Multilingual Plane (emoji, etc.) arrive as a SURROGATE PAIR: a high
    // surrogate (D800–DBFF) immediately followed by `\u` + a low surrogate
    // (DC00–DFFF). A surrogate on its own is not a valid char in Rust.
    fn parse_unicode_escape(&mut self) -> Result<char, String> {
        let first = self.parse_hex4()?;
        let code_point = match first {
            0xD800..=0xDBFF => {
                self.expect('\\')
                    .and_then(|()| self.expect('u'))
                    .map_err(|_| format!("high surrogate \\u{first:04X} not followed by \\u"))?;
                let second = self.parse_hex4()?;
                if !(0xDC00..=0xDFFF).contains(&second) {
                    return Err(format!(
                        "high surrogate \\u{first:04X} followed by non-low-surrogate \\u{second:04X}"
                    ));
                }
                0x10000 + ((first - 0xD800) << 10) + (second - 0xDC00)
            }
            0xDC00..=0xDFFF => return Err(format!("unpaired low surrogate \\u{first:04X}")),
            other => other,
        };
        // Every value reaching here is a valid scalar value, but char::from_u32
        // is the checked conversion — no `as` casts that could silently lie.
        char::from_u32(code_point).ok_or_else(|| format!("invalid code point {code_point:#X}"))
    }

    fn parse_hex4(&mut self) -> Result<u32, String> {
        let mut value = 0;
        for _ in 0..4 {
            let digit = self
                .chars
                .next()
                .and_then(|c| c.to_digit(16))
                .ok_or("expected 4 hex digits after \\u")?;
            value = value * 16 + digit;
        }
        Ok(value)
    }

    // Strict JSON number grammar:
    //   number = [ "-" ] int [ frac ] [ exp ]
    //   int    = "0" / ( digit1-9 *digit )      -- no leading zeros
    //   frac   = "." 1*digit                     -- "1." is invalid
    //   exp    = ( "e" / "E" ) [ "+" / "-" ] 1*digit
    fn parse_number(&mut self) -> Result<Value, String> {
        let mut raw = String::new();
        if self.chars.peek() == Some(&'-') {
            raw.push('-');
            self.chars.next();
        }
        match self.chars.peek() {
            Some('0') => {
                raw.push('0');
                self.chars.next();
                if matches!(self.chars.peek(), Some(c) if c.is_ascii_digit()) {
                    return Err("leading zeros are not allowed".to_string());
                }
            }
            Some(c) if c.is_ascii_digit() => self.take_digits(&mut raw),
            other => return Err(format!("expected digit, got {other:?}")),
        }
        if self.chars.peek() == Some(&'.') {
            raw.push('.');
            self.chars.next();
            self.require_digits(&mut raw, "after '.'")?;
        }
        if let Some(&e @ ('e' | 'E')) = self.chars.peek() {
            raw.push(e);
            self.chars.next();
            if let Some(&sign @ ('+' | '-')) = self.chars.peek() {
                raw.push(sign);
                self.chars.next();
            }
            self.require_digits(&mut raw, "in exponent")?;
        }
        // The grammar is already validated, so this parse only fails on range:
        // f64 turns 1e400 into infinity, which JSON cannot represent.
        let n: f64 = raw
            .parse()
            .map_err(|e| format!("invalid number '{raw}': {e}"))?;
        if n.is_finite() {
            Ok(Value::Number(n))
        } else {
            Err(format!("number '{raw}' is out of range for f64"))
        }
    }

    fn take_digits(&mut self, raw: &mut String) {
        while let Some(&c) = self.chars.peek() {
            if !c.is_ascii_digit() {
                break;
            }
            raw.push(c);
            self.chars.next();
        }
    }

    fn require_digits(&mut self, raw: &mut String, context: &str) -> Result<(), String> {
        let before = raw.len();
        self.take_digits(raw);
        if raw.len() == before {
            return Err(format!("expected digit {context} in '{raw}'"));
        }
        Ok(())
    }

    fn parse_array(&mut self) -> Result<Value, String> {
        self.expect('[')?;
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
                Some(',') => {}
                Some(']') => return Ok(Value::Array(items)),
                other => return Err(format!("expected ',' or ']', got {other:?}")),
            }
        }
    }

    fn parse_object(&mut self) -> Result<Value, String> {
        self.expect('{')?;
        let mut map = BTreeMap::new();
        self.skip_whitespace();
        if self.chars.peek() == Some(&'}') {
            self.chars.next();
            return Ok(Value::Object(map));
        }
        loop {
            self.skip_whitespace();
            // Check for the key's quote explicitly so `{"a":1,}` and `{a:1}`
            // get a clear error instead of being misread as a string.
            if self.chars.peek() != Some(&'"') {
                return Err(format!("expected string key, got {:?}", self.chars.peek()));
            }
            let key = self.parse_string()?;
            self.skip_whitespace();
            self.expect(':')?;
            let value = self.parse_value()?;
            map.insert(key, value); // duplicate keys: last one wins, as in most parsers
            self.skip_whitespace();
            match self.chars.next() {
                Some(',') => {}
                Some('}') => return Ok(Value::Object(map)),
                other => return Err(format!("expected ',' or '}}', got {other:?}")),
            }
        }
    }
}

// Parse a COMPLETE document: one value, optional surrounding whitespace, and
// nothing else. Without the end-of-input check, "nulljunk" and "[1]]" would
// silently "succeed" by ignoring everything after the first value.
fn parse(input: &str) -> Result<Value, String> {
    let mut parser = Parser::new(input);
    let value = parser.parse_value()?;
    parser.skip_whitespace();
    match parser.chars.peek() {
        None => Ok(value),
        Some(c) => Err(format!(
            "trailing characters after JSON value, starting at {c:?}"
        )),
    }
}

fn main() {
    println!("=== Exercise 24: JSON From Scratch ===");

    // Section 1: parsing scalars
    println!("\n--- Section 1: scalars ---");
    for input in ["null", "true", "false", "42", "-3.5", "6.02e23"] {
        println!("{input:?} -> {:?}", parse(input));
    }

    // Section 2: parsing a nested structure
    println!("\n--- Section 2: nested object/array ---");
    let input = r#"{"name":"Ferris","age":10,"languages":["rust","c","c++"],"active":true}"#;
    let value = parse(input).expect("the sample document is valid JSON");
    println!("parsed: {value:?}");

    // Section 3: round-tripping through Display back to a JSON string
    println!("\n--- Section 3: serialize back to text ---");
    let text = value.to_string();
    println!("serialized: {text}");
    println!(
        "re-parses to the same value: {}",
        parse(&text) == Ok(value.clone())
    );

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

    // Section 5: escapes — decoded on the way in, re-escaped on the way out
    println!("\n--- Section 5: escapes and Unicode ---");
    let escaped = r#"{"quote \"key\"":"tab\there, crab \ud83e\udd80, e\u0301"}"#;
    let decoded = parse(escaped).expect("valid escapes");
    println!("decoded:    {decoded:?}");
    println!("re-encoded: {decoded}");

    // Section 6: errors — the parser rejects anything that isn't strict JSON
    println!("\n--- Section 6: error handling ---");
    for bad in [
        "{not valid json",
        "[1]]",
        "01",
        "1.",
        r#"{"a":1,}"#,
        "\"\\ud800\"",
    ] {
        println!("{bad:?} -> {:?}", parse(bad));
    }
    let too_deep = "[".repeat(MAX_DEPTH + 1);
    println!("{} nested '[' -> {:?}", MAX_DEPTH + 1, parse(&too_deep));

    println!("\nNotes:");
    println!("  - This parser is what serde_json + #[derive(Serialize, Deserialize)] replace in practice.");
    println!("  - Value is a recursive enum — Array/Object hold more Values, just like exercise 16's Box<List>.");
    println!("  - BTreeMap (not HashMap) keeps object keys sorted so serialized output is deterministic.");
    println!(
        "  - Strictness matters: a lenient parser silently accepts input another parser rejects."
    );
    println!("  - A real project should use serde_json: faster, battle-tested, and exact number handling.");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn obj(pairs: &[(&str, Value)]) -> Value {
        Value::Object(
            pairs
                .iter()
                .map(|(k, v)| ((*k).to_string(), v.clone()))
                .collect(),
        )
    }

    fn s(text: &str) -> Value {
        Value::String(text.to_string())
    }

    #[test]
    fn parses_valid_documents() {
        let cases = [
            ("null", Value::Null),
            ("true", Value::Bool(true)),
            ("false", Value::Bool(false)),
            ("0", Value::Number(0.0)),
            ("-0", Value::Number(-0.0)),
            ("42", Value::Number(42.0)),
            ("-3.5", Value::Number(-3.5)),
            ("1e3", Value::Number(1000.0)),
            ("2.5E-1", Value::Number(0.25)),
            ("1e+2", Value::Number(100.0)),
            ("  \t\r\n 7 \n", Value::Number(7.0)),
            (r#""""#, s("")),
            (r#""plain""#, s("plain")),
            (
                r#""\" \\ \/ \b \f \n \r \t""#,
                s("\" \\ / \u{08} \u{0C} \n \r \t"),
            ),
            (r#""\u0041\u00e9""#, s("Aé")),
            (r#""\ud83e\udd80""#, s("🦀")),
            ("\"caf\u{e9} 🦀\"", s("café 🦀")),
            ("[]", Value::Array(vec![])),
            (
                "[ 1 , [2] ]",
                Value::Array(vec![
                    Value::Number(1.0),
                    Value::Array(vec![Value::Number(2.0)]),
                ]),
            ),
            ("{}", obj(&[])),
            (
                r#"{ "a" : 1 , "b" : [true, null] }"#,
                obj(&[
                    ("a", Value::Number(1.0)),
                    ("b", Value::Array(vec![Value::Bool(true), Value::Null])),
                ]),
            ),
            (r#"{"a":1,"a":2}"#, obj(&[("a", Value::Number(2.0))])),
        ];
        for (input, expected) in cases {
            assert_eq!(parse(input), Ok(expected), "input was {input:?}");
        }
    }

    #[test]
    fn rejects_invalid_documents() {
        let cases = [
            "",
            "   ",
            "nulljunk",
            "nul",
            "1 2",
            "[1]]",
            "{}x",
            "01",
            "-01",
            "1.",
            ".5",
            "-",
            "+1",
            "1e",
            "1e+",
            "1e400",
            "NaN",
            "[1,]",
            "[1 2]",
            "[",
            r#"{"a":1,}"#,
            r#"{x":1}"#,
            "{a:1}",
            r#"{"a" 1}"#,
            r#"{"a":}"#,
            r#""unterminated"#,
            r#""bad \x escape""#,
            r#""\u12""#,
            r#""\uZZZZ""#,
            r#""\ud800""#,
            r#""\ud800\u0041""#,
            r#""\udc00""#,
            "\"raw\nnewline\"",
            "\"raw\u{1}control\"",
            "'single quotes'",
        ];
        for input in cases {
            assert!(
                parse(input).is_err(),
                "should reject {input:?}, got {:?}",
                parse(input)
            );
        }
    }

    #[test]
    fn enforces_depth_limit() {
        let ok = format!("{}{}", "[".repeat(MAX_DEPTH), "]".repeat(MAX_DEPTH));
        assert!(parse(&ok).is_ok(), "exactly MAX_DEPTH levels should parse");

        let too_deep = format!("{}{}", "[".repeat(MAX_DEPTH + 1), "]".repeat(MAX_DEPTH + 1));
        assert!(parse(&too_deep).is_err());

        // The input that used to overflow the stack now fails cleanly.
        assert!(parse(&"[".repeat(200_000)).is_err());
        assert!(parse(&"{\"k\":".repeat(200_000)).is_err());
    }

    #[test]
    fn serializes_with_escaping() {
        let cases = [
            (s("a\"b"), r#""a\"b""#),
            (s("a\\b"), r#""a\\b""#),
            (s("line\nbreak\ttab\r"), r#""line\nbreak\ttab\r""#),
            (s("\u{08}\u{0C}\u{01}\u{1F}"), r#""\b\f\u0001\u001f""#),
            (s("crab 🦀 / é"), "\"crab 🦀 / é\""),
            (obj(&[("k\"q\\", Value::Null)]), r#"{"k\"q\\":null}"#),
            (Value::Number(42.0), "42"),
            (Value::Number(-0.5), "-0.5"),
            (Value::Number(f64::NAN), "null"),
            (Value::Number(f64::INFINITY), "null"),
            (Value::Number(f64::NEG_INFINITY), "null"),
        ];
        for (value, expected) in cases {
            assert_eq!(value.to_string(), expected, "value was {value:?}");
        }
    }

    #[test]
    fn round_trips_through_text() {
        let documents = [
            r#"{"name":"Ferris","age":10,"languages":["rust","c","c++"],"active":true}"#,
            r#"{"k\"q":"a\\b","nl":"line\nbreak","ctl":"\u0001","emoji":"\ud83e\udd80"}"#,
            r#"[null,true,false,0,-1.5,1e21,1e-7,"",[],{}]"#,
            r#"{"nested":{"deeper":[[{"x":[1,2,3]}]]}}"#,
        ];
        for doc in documents {
            let first = parse(doc).unwrap_or_else(|e| panic!("{doc:?} failed to parse: {e}"));
            let text = first.to_string();
            let second =
                parse(&text).unwrap_or_else(|e| panic!("re-parse of {text:?} failed: {e}"));
            assert_eq!(
                first, second,
                "round trip changed {doc:?} (serialized as {text:?})"
            );
            // Serializing is deterministic, so a second pass is byte-identical.
            assert_eq!(text, second.to_string());
        }
    }
}
