//! # 函数
//! Rust 的函数我们在之前已经见过不少，跟其他语言几乎没有什么区别。因此本章的学习之路将轻松和愉快，骚年们，请珍惜这种愉快，下一章你将体验到不一样的 Rust。
//! 在函数界，有一个函数只闻其名不闻其声，可以止小孩啼！在程序界只有 hello,world! 可以与之媲美，它就是 add 函数：
//!```
//!
//! fn add(i: i32, j: i32) -> i32 {
//! i + j
//! }
//!```
use std::fmt::Debug;
use common::print_start_end;

/// # 函数要点
/// + 函数名和变量名使用蛇形命名法(snake case)，例如 fn add_two() -> {}
/// + 函数的位置可以\[随便放\]，Rust 不关心我们在哪里定义了函数，只要有定义即可
/// + 每个函数参数都需要标注类型
#[print_start_end("函数要点")]
pub fn function_gist() {

}

/// # 函数参数
/// Rust 是强类型语言，因此需要你为每一个函数参数都标识出它的具体类型，例如：
#[print_start_end("函数参数")]
pub fn function_param() {

    fn another_function(x: i32, y: f32) {
        println!("The value of x is: {}", x);
        println!("The value of y is: {}", y);
    }
    another_function(5, 6.1);
}

/// # 函数返回
/// + 在上一章节语句和表达式中，我们有提到，在 Rust 中函数就是表达式，因此我们可以把函数的返回值直接赋给调用者。
/// + 函数 8的返回值就是函数体最后一条表达式的返回值，当然我们也可以使用 return 提前返回，下面的函数使用最后一条表达式来返回一个值：
#[print_start_end("函数返回")]
pub fn function_return() {
    fn plus_five(x:i32) -> i32 {
        x + 5
    }
    let x = plus_five(5);
    println!("The value of x is: {}", x);

}


/// # 无返回值()
/// 例如单元类型 ()，是一个零长度的元组。它没啥作用，但是可以用来表达一个函数没有返回值：
/// + 函数没有返回值，那么返回一个 ()
/// + 通过 ; 结尾的表达式返回一个 ()
#[print_start_end("无返回值()")]
pub fn function_non_return() {


    fn add(x:u32,y:u32)  {
        x + y;
    }
    fn add_r(x:u32,y:u32)->u32 {
        x + y
    }
}

/// # 永不返回的发散函数 !()
/// 当用 ! 作函数返回类型的时候，表示该函数永不返回( diverge function )，特别的，这种语法往往用做会导致程序崩溃的函数：
/// 表示该函数不能正常返回
#[print_start_end("永不返回的发散函数")]
pub fn function_diverge() {
    fn dead_end() -> ! {
        panic!("你已经到了穷途末路，崩溃吧！");
    }

    fn forever() -> ! {
        let mut x=1;
        loop {
            x+=1;
            println!("loop_");
        };
    }
    fn f(a:i32,b:i32)->bool{
        if a > b {
            true
        } else {
            panic!()
        }
    }
    fn sum(a: i32, b: i32) -> i32 {
        a+b;
        panic!()
    }
    /// 发散函数和空返回值函数不同，后者可以被返回：
    /// + 发散函数最大的用处就是用来通过 Rust 的类型检查。比如前面介绍 if 表达式时说过，如果存在多个条件分支，那么每个分支返回值的类型都要保持一致。可以使用发散函数调用的结果作为分支的返回，可以通过 Rust 的类型检查而不报错：
    /// + rust里说 Diverging function 是一种不会返回的函数，比如说 panic!() 还有死循环。
    /// + Diverging function 的返回类型就是 '!'，读作 diverges。
    /// + 它可以被转换为任意的类型：
    fn bar() {
       return ()
    }
    let a:() = bar();
    println!("在此可以看到函数 bar 的返回 {:?}",a);
    // dead_end();
    // forever();
}