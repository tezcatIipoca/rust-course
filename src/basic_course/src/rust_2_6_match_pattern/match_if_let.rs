use std::net::IpAddr;
use common::{inner_print, outer_print};

#[outer_print("match 和 if let")]
pub fn match_if_let() {
    //在 Rust 中，模式匹配最常用的就是 match 和 if let，先来看一个关于 match 的简单例子：
    enum Direction {
        East,
        West,
        South,
        North,
    }
    let dire = Direction::South;
    match dire {
        Direction::East => println!("E"),
        Direction::North | Direction::South => println!("N or S"),
        _ => println!("W")
    };
    //这里我们想去匹配 dire 对应的枚举类型，因此在 match 中用三个匹配分支来完全覆盖枚举变量 Direction 的所有成员类型，有以下几点值得注意
    /// + match 的匹配必须要穷举出所有可能，因此这里用 _ 来代表未列出的所有可能性
    /// + match 的每一个分支都必须是一个表达式，且所有分支的表达式最终返回值的类型必须相同
    /// + X | Y，类似逻辑运算符 或，代表该分支可以匹配 X 也可以匹配 Y，只要满足一个即可
    ///
    /// 其实 match 跟其他语言中的 switch 非常像，_ 类似于 switch 中的 default。
    ()
}

#[outer_print("match匹配")]
pub fn match_matching() {
    //首先来看看 match 的通用形式：
    // match target {
    //     模式1 => 表达式1,
    //     模式2 => {
    //         语句1;
    //         语句2;
    //         表达式2
    //     },
    //     _ => 表达式3
    // }

    //该形式清晰的说明了何为模式，何为模式匹配：将模式与 target 进行匹配，即为模式匹配，而模式匹配不仅仅局限于 match
    #[derive(Debug)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    let coin = Coin::Penny;
    let m = match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => { 5 }
        Coin::Dime => { 10 }
        Coin::Quarter => { 25 }
    };
    println!("coin:{:?}", m);
    ///当 match 表达式执行时，它将目标值 coin 按顺序依次与每一个分支的模式相比较，如果模式匹配了这个值，那么模式之后的代码将被执行。如果模式并不匹配这个值，将继续执行下一个分支。

    inner_print!("使用match表达式赋值");
    enum IpAddr {
        V4,
        V6,
    }
    let ip1 = IpAddr::V6;
    let ip_str = match ip1 {
        IpAddr::V4 => "127.0.0.1",
        _ => "::1",
    };
    println!("{}", ip_str);

    inner_print!("模式绑定");
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }
    enum Coins {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState), // 25美分硬币
    }
    fn value_in_cents(coin: Coins) -> u8 {
        match coin {
            Coins::Penny => 1,
            Coins::Nickel => 5,
            Coins::Dime => 10,
            Coins::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }
    //上面代码中，在匹配 Coin::Quarter(state) 模式时，我们把它内部存储的值绑定到了 state 变量上，因此 state 变量就是对应的 UsState 枚举类型。

    //再来看一个更复杂的例子：
    enum Action {
        Say(String),
        MoveTo(i32, i32),
        ChangeColorRGB(u16, u16, u16),
    }

    fn main() {
        let actions = [
            Action::Say("Hello Rust".to_string()),
            Action::MoveTo(1, 2),
            Action::ChangeColorRGB(255, 255, 0),
        ];
        for action in actions {
            match action {
                Action::Say(s) => {
                    println!("{}", s);
                }
                Action::MoveTo(x, y) => {
                    println!("point from (0, 0) move to ({}, {})", x, y);
                }
                Action::ChangeColorRGB(r, g, _) => {
                    println!("change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
                             r, g,
                    );
                }
            }
        }
    }
    inner_print!("穷尽匹配");
    //and so on
    inner_print!("_ 通配符");
    ///当我们不想在匹配时列出所有值的时候，可以使用 Rust 提供的一个特殊模式，
    /// 我们不必一个一个列出所有值, 因为可以使用特殊的模式 _ 替代
    let su8v = 0u8;
    match su8v {
        1 => println!("one"),
        3 => println!("three"),
        4 => println!("five"),
        _ => {}
    }
    //除了_通配符，用一个变量来承载其他情况也是可以的。
    #[derive(Debug)]
    enum Direction {
        East,
        West,
        North,
        South,
    }
    fn main1() {
        let dire = Direction::South;
        match dire {
            Direction::East => println!("East"),
            other => println!("other direction: {:?}", other),
        };
    }
}

#[outer_print("if let匹配")]
pub fn if_let() {
    ///有时会遇到只有一个模式的值需要被处理，其它值直接忽略的场景，如果用 match 来处理就要写成下面这样：
    let v = Some(3u8);
    match v {
        Some(3) => println!("three"),
        _ => (),
    };
    //我们只想要对 Some(3) 模式进行匹配, 不想处理任何其他 Some<u8> 值或 None 值。但是为了满足 match 表达式（穷尽性）的要求，写代码时必须在处理完这唯一的成员后加上 _ => ()，这样会增加不少无用的代码。
    //我们完全可以用 if let 的方式来实现：
    if let Some(3) = v {
        println!("three_");
    }
    ///当你只要匹配一个条件，且忽略其他条件时就用 if let ，否则都用 match。
    /// if和if let区别  if 匹配布尔表达式 if let 匹配模式
    ();
}

#[outer_print("matches宏")]
pub fn matches_macro() {
    enum MyEnum {
        Foo,
        Bar,
    }
    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    //现在如果想对 v 进行过滤，只保留类型是 MyEnum::Foo 的元素，你可能想这么写：
    // v.iter().filter(|x| x == MyEnum::Foo);
    //但是，实际上这行代码会报错，因为你无法将 x 直接跟一个枚举成员进行比较。
    //好在，你可以使用 match 来完成，但是会导致代码更为啰嗦，是否有更简洁的方式？答案是使用 matches!：
    let filter = v.iter().filter(|x| matches!(x, MyEnum::Foo));
}

#[outer_print("变量遮蔽")]
pub fn variable_shadowing() {
    //无论是 match 还是 if let，这里都是一个新的代码块，而且这里的绑定相当于新变量，如果你使用同名变量，会发生变量遮蔽:
    let age = Some(30);
    println!("在匹配前，age是{:?}", age);
    if let Some(age) = age {
        println!("匹配出来的age是{}", age);
    }

    println!("在匹配后，age是{:?}", age);

    //对于 match 类型也是如此:
    let age = Some(30);
    println!("在匹配前，age是{:?}", age);
    match age {
        Some(age) => println!("匹配出来的age是{}", age),
        _ => ()
    }
    println!("在匹配后，age是{:?}", age);
}