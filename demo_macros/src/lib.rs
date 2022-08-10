extern crate proc_macro;
mod util;

use quote::quote;
use syn::PathArguments::None;
use util::*;
use proc_macro::TokenStream;
use std::str::FromStr;



#[proc_macro_attribute]
pub fn cache_aop(_attr: TokenStream, _input: TokenStream) -> TokenStream {
    let attr_str = _attr.clone().to_string();
    let input_str = _input.clone().to_string();
    let param = attr_split_to_map(&attr_str);
    println!("attr_str : {}", attr_str);
    println!("input_str : {}", input_str);
    println!("param : {:?}", param);
    let command = match param.get("command") {
        Some(get) if get.to_lowercase() == "get" => {
            match param.get("key") {
                Some(k) => Some(Command::GET(Some(k.clone()))),
                None => {
                    println!("GET command has no key!");
                    None
                },
            }

        },
        Some(list) if list.to_lowercase() == "list" => {
            match param.get("key") {
                Some(k) => Some(Command::LIST(Some(k.clone()))),
                None => {
                    println!("LIST command has no key!");
                    None
                },
            }
        },
        Some(hash) if hash.to_lowercase() == "hash" => {
            let k = match param.get("key") {
                Some(k) => Some(k.clone()),
                None => {
                    println!("HASH command has no key!");
                    None
                },
            };
            let f = match param.get("field") {
                Some(f) => Some(f.clone()),
                None => {
                    println!("HASH command has no field!");
                    None
                },
            };
            match (k,v) {
                (Some(_k), Some(_f)) => Some(Command::HASH(Some(_k), Some(_f))),
                None => None
            }
        },
        _ => panic!("unknown command!")
    };

    let result_token_stream = proc_macro2::TokenStream::from_str(&input_str).unwrap();
    // return TokenStream::from(quote!{fn dummy(){}});
    return TokenStream::from(result_token_stream);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
