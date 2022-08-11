# asura_macros

通过rust宏实现类似aop的功能
> 原本想实现查询redis缓存的aop实现, 最后只是简单打印一句增强语句,最终目的也达到了,仅供参考

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
