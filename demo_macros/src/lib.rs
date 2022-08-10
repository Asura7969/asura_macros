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
    // let code_str = format!(
    //     "pub fn aop() \n{{let v_op = container.get(\"{key}\"); println!(\"{{}}\", v_op.unwrap());}}    \n{src_code}",
    //     key = _k,
    //     src_code = input_str
    // );

    let code_str = format!(
        "fn aop() \n{{println!(\"{{}}\", {p1});}}    \n{src_code}",
        p1 = "1",
        src_code = input_str
    );
    println!("code_str : \n{}", code_str);
    let result_token_stream = proc_macro2::TokenStream::from_str(&code_str).unwrap();
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
