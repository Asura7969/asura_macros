use std::{collections::HashMap};

/// command
const GET:&str = "get";
const LIST:&str = "list";
const HASH:&str = "hash";


pub enum Command {
    GET,
    LIST,
    HASH
}

pub struct CommandImpl {
    key: String,
    field: Option<String>,
    tp: Command
}


impl CommandImpl {
    pub fn get_key(&self) -> String {
        self.key.clone()
    }
}


impl Command {

    pub fn from(param_ref: &HashMap<String, String>) -> Option<CommandImpl> {
        match param_ref.get("command") {
            Some(get) if get.to_lowercase() == GET => {
                let c_op = match param_ref.get("key") {
                    Some(k) => Some(CommandImpl {key: k.to_string(), field: None, tp: Command::GET}),
                    _ => {
                        println!("GET command has no key!");
                        None
                    },
                };
                c_op
            },
            Some(list) if list.to_lowercase() == LIST => {
                let c_op = match param_ref.get("key") {
                    Some(k) => Some(CommandImpl {key: k.to_string(), field: None, tp: Command::LIST}),
                    _ => {
                        println!("LIST command has no key!");
                        None
                    },
                };
                c_op
            },
            Some(hash) if hash.to_lowercase() == HASH => {
                let k = match param_ref.get("key") {
                    Some(k) => Some(k.to_string()),
                    _ => {
                        println!("HASH command has no key!");
                        None
                    },
                };
                let f = match param_ref.get("field") {
                    Some(f) => Some(f.to_string()),
                    _ => {
                        println!("HASH command has no field!");
                        None
                    },
                };
                if k.is_some() && f.is_some() {
                    Some(CommandImpl {key: k.unwrap(), field: f, tp: Command::HASH})
                } else {
                    None
                }
            },
            _ => {
                println!("unknown command!");
                None
            }
        }
    }
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
