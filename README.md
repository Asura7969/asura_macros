# asura_macros

通过rust宏实现类似aop的功能
> 原本想实现查询redis缓存的aop实现, 最后只是简单打印一句增强语句,最终目的也达到了,仅供参考

自定义示例:
> 此处以`axum web`框架为背景,增加路由层的handler缓存查询
```rust
    // ...
    let app = Router::new()
        .route("/user_by_id/:id", get(query_user_by_id))
    
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();


#[cache_aop(command="hash",key="key1", field="f1")]
pub async fn query_user_by_id(Extension(state): Extension<AppState>,
                              Path(id): Path<i64>) -> Result<RestJson<User>> {
    // 自定义宏添加的查询缓存逻辑
    
    // query_user_by_id 方法本身查询db的逻辑
}


/// 宏定义
#[proc_macro_attribute]
pub fn cache_aop(_attr: TokenStream, _input: TokenStream) -> TokenStream {
    let attr_str = _attr.clone().to_string();
    // query_user_by_id 方法(字符串)
    let input_str = _input.clone().to_string();
    // 自定义宏参数
    let param = attr_split_to_map(&attr_str);
    
    // 伪代码
    let code_str = format!("
        let mut con = state.redis.unwrap().get_async_connection().await?;\n
        redis::cmd("#GET")
        .arg(&[id.to_string()])
        .query_async(&mut con)
        .await {
            Ok(json) => // 缓存查询到数据直接返回
            
        }"
    );
    // 否则查询db或其他外部存储

    // 使自定义代码在handler之前执行
    expand_str.insert_str(start_pos, code_str.as_str());
    

    let result_token_stream = proc_macro2::TokenStream::from_str(&expand_str).unwrap();
    return TokenStream::from(result_token_stream);
}
```




实现方式如下:
* 定义过程宏
* 获取待执行的方法
* 在待执行方法中通过字符串形式扩展code
* 返回增强后的方法

> 本示例只是加了一行`println`语句。也可获取方法参数，对参数进行调整，改变后续业务逻辑等

> 实现方式较为粗暴,通用性较弱... 未完待续?
# 参考
[450220020/autocall](https://github.com/450220020/autocall)

[博客](https://www.cnblogs.com/praying/p/14520507.html)
