//! 语句和表达式
//! Rust 的函数体是由一系列语句组成，最后由一个表达式来返回值，例如：
//! ```
//!
//! fn add_with_extra(x: i32, y: i32) -> i32 {
//! let x = x + 1; // 语句
//! let y = y + 5; // 语句
//! x + y // 表达式
//! }
//! ```
//! 语句会执行一些操作但是不会返回一个值，而表达式会在求值后返回一个值，因此在上述函数体的三行代码中，前两行是语句，最后一行是表达式。
//! 对于 Rust 语言而言，这种基于语句（statement）和表达式（expression）的方式是非常重要的，你需要能明确的区分这两个概念, 但是对于很多其它语言而言，这两个往往无需区分。基于表达式是函数式语言的重要特征，表达式总要返回值。


use common::outer_print;

/// # 语句
///
#[outer_print("语句")]
pub fn statement() {
    let a = 8;
    let b: Vec<f64> = Vec::new();
    let (a, c) = ("hi", false);
    //以上都是语句，它们完成了一个具体的操作，但是并没有返回值，因此是语句。

    // 由于 let 是语句，因此不能将 let 语句赋值给其它值，如下形式是错误的：
    // let b = (let a = 8);
}
/// # 表达式
/// + 表达式会进行求值，然后返回一个值。例如 5 + 6，在求值后，返回值 11，因此它就是一条表达式。
/// + 表达式可以成为语句的一部分，例如 let y = 6 中，6 就是一个表达式，它在求值后返回一个值 6（有些反直觉，但是确实是表达式）。
/// + 调用一个函数是表达式，因为会返回一个值，调用宏也是表达式，用花括号包裹最终返回一个值的语句块也是表达式，总之，能返回值，它就是表达式:
#[outer_print("表达式")]
pub fn expression() {
    //上面使用一个语句块表达式将值赋给 y 变量，语句块长这样：
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    ///+ 该语句块是表达式的原因是：它的最后一行是表达式，返回了 x + 1 的值，注意 x + 1 不能以分号结尾，否则就会从表达式变成语句， 表达式不能包含分号。这一点非常重要，一旦你在表达式后加上分号，它就会变成一条语句，再也不会返回一个值，请牢记！
    ///+ 最后，表达式如果不返回任何值，会隐式地返回一个 () 。
    assert_eq!(ret_unit_type(), ());
    fn ret_unit_type() {
        let x = 1;
        // if 语句块也是一个表达式，因此可以用于赋值，也可以直接返回
        // 类似三元运算符，在Rust里我们可以这样写
        let y = if x % 2 == 1 {
            "odd"
        } else {
            "even"
        };
        // 或者写成一行
        let z = if x % 2 == 1 { "odd" } else { "even" };
    }

}