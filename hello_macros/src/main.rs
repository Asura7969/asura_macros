extern crate demo_macros;
use demo_macros::cache_aop;

#[derive(Debug)]
struct Param {
    pa: String,
    pb: String,
}

#[cache_aop(command="hash",key="key1", field="f1")]
fn it_works(param: Param) {
    println!("{:?}", param);
    let _result = 2 + 2;
}


fn main() {
    it_works(Param {
        pa: "参数-1".to_string(),
        pb: "参数-2".to_string()
    });
    println!("Hello, world!");
}
