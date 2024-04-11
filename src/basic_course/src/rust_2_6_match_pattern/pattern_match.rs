use common::{inner_print, outer_print};

#[outer_print("所有可能用到模式的地方")]
pub fn match_use() {
    inner_print!("match分支");
    //  match VALUE {
    //       PATTERN => EXPRESSION,
    //       PATTERN => EXPRESSION,
    //       PATTERN => EXPRESSION,
    //   }

    //如上所示，match 的每个分支就是一个模式，因为 match 匹配是穷尽式的，因此我们往往需要一个特殊的模式 _，来匹配剩余的所有情况：

    //  match VALUE {
    //      PATTERN => EXPRESSION,
    //      PATTERN => EXPRESSION,
    //       _ => EXPRESSION,
    //  }
    let v = Some(3);
    match v {
        None => {
            println!("None");
        }
        Some(2) => {
            println!("2")
        }
        Some(other) => {
            println!("{:?},{:?}", v, other);
        }
    }
    inner_print!("if let 分支");
    //if let 往往用于匹配一个模式，而忽略剩下的所有模式的场景：
    // if let PATTERN = SOME_VALUE {}
    if let Some(x) = v {
        println!("{:?}", x);
    }

    inner_print!("while let条件循环");
    //一个与 if let 类似的结构是 while let 条件循环，它允许只要模式匹配就一直进行 while 循环。下面展示了一个使用 while let 的例子：
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    while let Some(top) = vec.pop() {
        println!("{}", top);
    }

    inner_print!("for循环");
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
    //这里使用 enumerate 方法产生一个迭代器，该迭代器每次迭代会返回一个 (索引，值) 形式的元组，然后用 (index,value) 来匹配。
    ///  let 语句
    // let PATTERN = EXPRESSION;
    // 是的， 该语句我们已经用了无数次了，它也是一种模式匹配：
    // let x = 5;
    // 这其中，x 也是一种模式绑定，代表将匹配的值绑定到变量 x 上。因此，在 Rust 中,变量名也是一种模式，只不过它比较朴素很不起眼罢了。
    // let (x, y, z) = (1, 2, 3);
    // 模式匹配要求两边的类型必须相同，否则就会导致下面的报错：

    inner_print!("函数参数");
    //函数参数也是模式：
    fn foo(x: i32) {
        // 代码
    }
    // 其中 x 就是一个模式，你还可以在参数中匹配元组：
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }
    let point = (3, 5);
    print_coordinates(&point);
    //&(3, 5) 会匹配模式 &(x, y)，因此 x 得到了 3，y 得到了 5。

    inner_print!("let 和 if let");
    //对于以下代码，编译器会报错：
    //let Some(x) = some_option_value;
    //因为右边的值可能不为 Some，而是 None，这种时候就不能进行匹配，也就是上面的代码遗漏了 None 的匹配。
    ///类似 let , for和match 都必须要求完全覆盖匹配，才能通过编译( 不可驳模式匹配 )
    // 但是对于 if let，就可以这样使用：
    //  if let Some(x) = some_option_value {
    //     println!("{}", x);
    //  }
    // 因为 if let 允许匹配一种模式，而忽略其余的模式( 可驳模式匹配 )。
    ///模式匹配有可驳模式和不可驳模式两种。 if let 和 while let 就属于可驳模式匹配。
    ();
}