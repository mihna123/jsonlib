use jsonlib;
use std::fs;
use jsonlib::value::Value;

#[test]
fn test_parsing_01() {
    let data = fs::read_to_string("tests/data.json").expect("Should be able to open");
    let obj = jsonlib::parse(data.as_str()).expect("should parse no problem");
    if let Value::Object(o) = &obj {
        if let Value::Object(b) = &o["glossary"] {
            assert_eq!(b["title"], Value::String("example glossary".to_string()) );
        }
    } else {
        panic!("Parsed value was not object");
    }
}

#[test]
fn test_parsing_02() {
    let data = fs::read_to_string("tests/data.json").expect("Should be able to open");
    let obj = jsonlib::parse(data.as_str()).expect("should parse no problem");
    if let Value::Object(o) = &obj {
        if let Value::Object(glossary) = &o["glossary"] {
            if let Value::Object(gloss_div) = &glossary["GlossDiv"] {
                assert_eq!(gloss_div["title"], Value::String("S".to_string()));
            }
        }
    } else {
        panic!("Value is not an object!");
    }

}
