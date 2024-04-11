use common::{inner_print, outer_print};

#[outer_print("匹配字面值")]
pub fn match_literal() {
    let x = 1;
    match x {
        1 => { println!("one") }
        2 => { println!("two") }
        3 => { println!("three") }
        _ => { println!("anything") }
    }
}

#[outer_print("匹配命名变量")]
pub fn match_named_variable() {
    //在 match 中，我们有讲过变量遮蔽的问题，这个在匹配命名变量时会遇到：
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),

        //第二个匹配分支中的模式引入了一个新变量 y，它会匹配任何 Some 中的值。因为这里的 y 在 match 表达式的作用域中，而不是之前 main 作用域中，所以这是一个新变量，不是开头声明为值 10 的那个 y。
        Some(y) => println!("matched,y={:?}", y),

        //这个新的 y 绑定会匹配任何 Some 中的值，在这里是 x 中的值。因此这个 y 绑定了 x 中 Some 内部的值。这个值是 5，所以这个分支的表达式将会执行并打印出 Matched，y = 5。
        _ => println!()
    }
    //一旦 match 表达式执行完毕，其作用域也就结束了，同理内部 y 的作用域也结束了
    //最后的 println! 会打印 at the end: x = Some(5), y = 10。
    println!("at the end: x = {:?}, y = {:?}", x, y);
}

#[outer_print("单分支多模式")]
pub fn single_branch_multi_mode() {
    // 在 match 表达式中，可以使用 | 语法匹配多个模式，它代表 或的意思。
    // 例如，如下代码将 x 的值与匹配分支相比较，第一个分支有 或 选项，意味着如果 x 的值匹配此分支的任何一个模式，它就会运行：
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

#[outer_print("通过序列 ..= 匹配值的范围")]
pub fn match_range() {
    // 在数值类型中我们有讲到一个序列语法，该语法不仅可以用于循环中，还能用于匹配模式。
    // ..= 语法允许你匹配一个闭区间序列内的值。在如下代码中，当模式匹配任何在此序列内的值时，该分支会执行：
    let x = 5;
    match x {
        1..=5 => println!("1-5"),
        _ => println!("something else")
    }
    //序列只允许用于数字或字符类型，原因是：它们可以连续，同时编译器在编译期可以检查该序列是否为空，字符和数字值是 Rust 中仅有的可以用于判断是否为空的类型。
    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}


#[outer_print("解构并且分解值")]
pub fn structure_value() {
    //也可以使用模式来解构结构体、枚举、元组、数组和引用。
    inner_print!("解构结构体");
    {
        //下面代码展示了如何用 let 解构一个带有两个字段 x 和 y 的结构体 Point：
        struct Point {
            x: i32,
            y: i32,
        }
        let p = Point { x: 0, y: 7 };
        let Point { x: a, y: b } = p;
        println!("a:{:?} ,b:{:?}", a, b);
        //这段代码创建了变量 a 和 b 来匹配结构体 p 中的 x 和 y 字段，这个例子展示了模式中的变量名不必与结构体中的字段名一致。不过通常希望变量名与字段名一致以便于理解变量来自于哪些字段。


        //因为变量名匹配字段名是常见的，同时因为 let Point { x: x, y: y } = p; 中 x 和 y 重复了，所以对于匹配结构体字段的模式存在简写：只需列出结构体字段的名称，则模式创建的变量会有相同的名称
        let p = Point { x: 0, y: 7 };
        let Point { x, y } = p;
        assert_eq!(0, x);
        assert_eq!(7, y);

        //也可以使用字面值作为结构体模式的一部分进行解构，而不是为所有的字段创建变量。这允许我们测试一些字段为特定值的同时创建其他字段的变量。
        let p = Point { x: 0, y: 7 };
        match p {
            Point { x, y: 0 } => println!("On the x axis at {}", x),
            Point { x: 0, y } => println!("On the y axis at {}", y),
            Point { x, y } => println!("On neither axis: ({}, {})", x, y),
        }
    }
    inner_print!("解构枚举");
    {
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }
        let msg = Message::ChangeColor(0, 160, 255);
        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructure.")
            }
            Message::Move { x, y } => {
                println!(
                    "Move in the x direction {} and in the y direction {}",
                    x,
                    y
                );
            }
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => {
                println!(
                    "Change the color to red {}, green {}, and blue {}",
                    r,
                    g,
                    b
                )
            }
        }
        //模式匹配一样要类型相同，因此匹配 Message::Move{1,2} 这样的枚举值，就必须要用 Message::Move{x,y} 这样的同类型模式才行。
        //对于像 Message::Quit 这样没有任何数据的枚举成员，不能进一步解构其值。只能匹配其字面值 Message::Quit，因此模式中没有任何变量。
    }

    inner_print!("解构嵌套的结构体和枚举");
    {
        enum Color {
            Rgb(i32, i32, i32),
            Hsv(i32, i32, i32),
        }
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(Color),
        }
        let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
        match msg {
            Message::ChangeColor(Color::Rgb(r, g, b)) => {
                println!(
                    "Change the color to red {}, green {}, and blue {}",
                    r,
                    g,
                    b
                )
            }
            Message::ChangeColor(Color::Hsv(h, s, v)) => {
                println!(
                    "Change the color to hue {}, saturation {}, and value {}",
                    h,
                    s,
                    v
                )
            }
            _ => ()
        }

    }

    inner_print!("解构结构体和元组");
    {
        struct Point {
            x: i32,
            y: i32,
        }

        let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });

    }
    inner_print!("解构数组");
    {
        //定长数组
        //不定长数组
        //todo

    }
}

#[outer_print("忽略模式中的值")]
pub fn ignore_mode_value() {
    // 有时忽略模式中的一些值是很有用的，比如在 match 中的最后一个分支使用 _ 模式匹配所有剩余的值。
    /// 你也可以在另一个模式中使用 _ 模式，使用一个以下划线开始的名称，或者使用 .. 忽略所剩部分的值。
    inner_print!("使用 _ 忽略整个值");
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }

    foo(3, 4);
    //在一些情况下忽略函数参数会变得特别有用，比如实现特征时，当你需要特定类型签名但是函数实现并不需要某个参数时。此时编译器就不会警告说存在未使用的函数参数，就跟使用命名参数一样。

    inner_print!("使用嵌套的 _ 忽略部分值");
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    // 第一个匹配分支，我们不关心里面的值，只关心元组中两个元素的类型，因此对于 Some 中的值，直接进行忽略。
    // 剩下的形如 (Some(_),None)，(None, Some(_)), (None,None) 形式，都由第二个分支 _ 进行分配。
    println!("setting is {:?}", setting_value);

    /// 还可以在一个模式中的多处使用下划线来忽略特定值，如下所示，这里忽略了一个五元元组中的第二和第四个值：
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }

    inner_print!("使用下划线开头忽略未使用的变量");
    //比如你正在设计原型或刚刚开始一个项目。这时你希望告诉 Rust 不要警告未使用的变量，为此可以用下划线作为变量名的开头：
    let _x = 5;
    let y = 10;
    ///注意, 只使用 _ 和使用以下划线开头的名称有些微妙的不同：比如 _x 仍会将值绑定到变量，而 _ 则完全不会绑定。
    //只使用下划线本身，则并不会绑定值，因为 s 没有被移动进 _：
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);

    inner_print!("用 .. 忽略剩余值");
    /// 对于有多个部分的值，可以使用 .. 语法来只使用部分值而忽略其它值，这样也不用再为每一个被忽略的值都单独列出下划线。
    /// .. 模式会忽略模式中剩余的任何没有显式匹配的值部分。
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }
    //还可以用 .. 来忽略元组中间的某些值：
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }
    //然而使用 .. 必须是无歧义的。如果期望匹配和忽略的值是不明确的，Rust 会报错。下面代码展示了一个带有歧义的 .. 例子，因此不能编译：
    //fn main() {
    //     let numbers = (2, 4, 8, 16, 32);
    //
    //     match numbers {
    //         (.., second, ..) => {
    //             println!("Some numbers: {}", second)
    //         },
    //     }
    // }
    //Rust 无法判断，second 应该匹配 numbers 中的第几个元素，因此这里使用两个 .. 模式，是有很大歧义的！
}

#[outer_print("匹配守卫提供的额外条件")]
pub fn match_guard() {
    ///匹配守卫（match guard）是 一个位于 match 分支模式之后的额外 if 条件，它能为分支模式提供更进一步的匹配条件。
    let num = Some(3);
    match num {
        Some(x) if x < 5 => { println!("less than five:{}", x) }
        Some(x) => println!("{}", x),
        None => (),
    }
    //这个例子会打印出 less than five: 4。当 num 与模式中第一个分支匹配时，Some(4) 可以与 Some(x) 匹配，接着匹配守卫检查 x 值是否小于 5，因为 4 小于 5，所以第一个分支被选择。
    //模式中无法提供类如 if x < 5 的表达能力，我们可以通过匹配守卫的方式来实现。

    // 在之前，我们提到可以使用匹配守卫来解决模式中变量覆盖的问题，那里 match 表达式的模式中新建了一个变量而不是使用 match 之外的同名变量。
    // 内部变量覆盖了外部变量，意味着此时不能够使用外部变量的值，下面代码展示了如何使用匹配守卫修复这个问题。
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }


    let x = 4;
    let y = false;

    ///也可以在匹配守卫中使用 或 运算符 | 来指定多个模式，同时匹配守卫的条件会作用于所有的模式。
    match x {
        //这个匹配条件表明此分支只匹配 x 值为 4、5 或 6 同时 y 为 true 的情况。
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

#[outer_print("@捕获绑定")]
/// @（读作 at）运算符允许为一个字段绑定另外一个变量
pub fn at_bind() {

    // 我们希望测试 Message::Hello 的 id 字段是否位于 3..=7 范围内，同时也希望能将其值绑定到 id_variable 变量中以便此分支中相关的代码可以使用它。
    // 我们可以将 id_variable 命名为 id，与字段同名，不过出于示例的目的这里选择了不同的名称。
    #[derive(Debug)]
    enum Message{
        Hello{id:i32}

    };
    let message = Message::Hello { id:3 };
    match message {
        Message::Hello { id:id_variable @ 0..=7 } => {
            println!("id_var:{:?}", id_variable);
            println!("{:?}",message);
        },
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        },
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        },
    };

    inner_print!("@前绑定后解构(Rust 1.56 新增)");
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    let p @ Point {x: px, y: py } = Point {x: 10, y: 23};
    let Point { x, y } = Point {x: 11, y: 22};
    let p1=Point{x,y};
    println!("x: {}, y: {}", px, py);
    println!("{:?}", p);
    println!("{:?}", p1);
    let point = Point {x: 10, y: 5};
    if let p @ Point {x: 10, y} = point {
        println!("x is 10 and y is {} in {:?}", y, p);
    } else {
        println!("x was not 10 :(");
    }

    inner_print!("@新特性(Rust 1.53 新增)");
    match 1 {
        num @ (1 | 2) => {
            println!("num:{}", num);
        }
        _ => {}
    }
    let (x,y)=(1,2);
    println!("{}",x+y);
    let p@(x,y)=(1,2);
    println!("{}",x+y);
}

