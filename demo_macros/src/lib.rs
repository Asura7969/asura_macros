extern crate proc_macro;
mod util;

use quote::quote;
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
    let mut command = String::new();
    if let Some(c) = param.get("command") {
        first_bean = r.clone();
    }

    let code_str = "let tmp_v = 1";
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
