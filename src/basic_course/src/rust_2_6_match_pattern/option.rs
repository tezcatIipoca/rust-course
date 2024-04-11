use common::outer_print;

#[outer_print("解构 Option")]
pub fn structure_option(){
    //在枚举那章，提到过 Option 枚举，它用来解决 Rust 中变量是否有值的问题，定义如下：
    enum Option<T> {
        Some(T),
        None,
    }
    /// 简单解释就是：一个变量要么有值：Some(T), 要么为空：None。
    //因为 Option，Some，None 都包含在 prelude 中，因此你可以直接通过名称来使用它们，而无需以 Option::Some 这种形式去使用，总之，千万不要因为调用路径变短了，就忘记 Some 和 None 也是 Option 底下的枚举成员！
    ();
}

#[outer_print("匹配 Option<T>")]
pub fn match_option(){
    //使用 Option<T>，是为了从 Some 中取出其内部的 T 值以及处理没有值的情况，为了演示这一点，下面一起来编写一个函数，它获取一个 Option<i32>，如果其中含有一个值，将其加一；如果其中没有值，则函数返回 None 值：
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    ///传入参数 Some(5)
    //None => None,

    //首先是匹配 None 分支，因为值 Some(5) 并不匹配模式 None，所以继续匹配下一个分支。
    //Some(i) => Some(i + 1),
    //Some(5) 与 Some(i) 匹配吗？当然匹配！它们是相同的成员。i 绑定了 Some 中包含的值，因此 i 的值是 5。接着匹配分支的代码被执行，最后将 i 的值加一并返回一个含有值 6 的新 Some。

    ///传入参数 None
    //接着考虑下 plus_one 的第二个调用，这次传入的 x 是 None， 我们进入 match 并与第一个分支相比较。
    //None => None,
    ();
}