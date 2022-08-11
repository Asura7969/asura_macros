extern crate proc_macro;
mod util;

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
    let param_ref = &param;
    let command_op = Command::from(param_ref);
    let command = command_op.unwrap();
    let _k = (&command).get_key();
    let start_pos = input_str.find("{").unwrap() + 1;
    let mut expand_str = _input.clone().to_string();
    let code_str = format!(
        "println!(\"{{}}\", \"{p1}\");    \n",
        p1 = "打印-宏扩展代码",
    );

    expand_str.insert_str(start_pos, code_str.as_str());

    println!("code_str : \n{}", expand_str);
    let result_token_stream = proc_macro2::TokenStream::from_str(&expand_str).unwrap();
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
