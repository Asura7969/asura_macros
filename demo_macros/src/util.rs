use std::{collections::HashMap};

pub enum Command {
    GET(Option<String>),
    LIST(Option<String>),
    HASH(Option<String>, Option<String>)
}

pub fn attr_split_to_map(attr_str: &str) -> HashMap<String, String> {
    attr_str
        .replace("/", "")
        .replace('\n', "")
        .replace('\\', "")
        .trim()
        .to_string();
    let attr_split = attr_str.split("\",").map(|s| s.replace("\"", ""));
    let mut attr_map = HashMap::<String, String>::new();
    for attr_mate in attr_split {
        let attr_sp = attr_mate.split_once("=");
        if let Some((key, val)) = attr_sp {
            attr_map.insert(key.trim().to_string(), val.trim().to_string());
        }
    }
    return attr_map;
}
