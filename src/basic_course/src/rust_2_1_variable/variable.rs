//! # 变量绑定与解构
//! [https://course.rs/basic/variable.html](https://course.rs/basic/variable.html)
//! 鉴于本书的目标读者（别慌，来到这里就说明你就是目标读者）已经熟练掌握其它任意一门编程语言，
//! 因此这里就不再对何为变量进行赘述，让我们开门见山来谈谈，为何 Rust 选择了手动设定变量的可变性。
//! ## 为何要手动设置变量的可变性？
//! + 在其它大多数语言中，要么只支持声明可变的变量，要么只支持声明不可变的变量( 例如函数式语言 )，前者为编程提供了灵活性，后者为编程提供了安全性，而 Rust 比较野，选择了两者我都要，既要灵活性又要安全性。
//! + 能想要学习 Rust，说明我们的读者都是相当有水平的程序员了，你们应该能理解一切选择皆是权衡，那么两者都要的权衡是什么呢？这就是 Rust 开发团队为我们做出的贡献，两者都要意味着 Rust 语言底层代码的实现复杂度大幅提升，因此 Salute to The Rust Team!
//! + 除了以上两个优点，还有一个很大的优点，那就是运行性能上的提升，因为将本身无需改变的变量声明为不可变在运行期会避免一些多余的 runtime 检查。
//! ## 变量命名
//! + 在命名方面，和其它语言没有区别，不过当给变量命名时，需要遵循 Rust 命名规范。
//! + Rust 语言有一些关键字（keywords），和其他语言一样，这些关键字都是被保留给 Rust 语言使用的，因此，它们不能被用作变量或函数的名称。在 附录 A 中可找到关键字列表。
//! ## 变量绑定
//! + 在其它语言中，我们用 var a = "hello world" 的方式给 a 赋值，也就是把等式右边的 "hello world" 字符串赋值给变量 a ，而在 Rust 中，我们这样写： let a = "hello world" ，同时给这个过程起了另一个名字：变量绑定。
//! + 为何不用赋值而用绑定呢（其实你也可以称之为赋值，但是绑定的含义更清晰准确）？这里就涉及 Rust 最核心的原则——所有权，简单来讲，任何内存对象都是有主人的，而且一般情况下完全属于它的主人，绑定就是把这个对象绑定给一个变量，让这个变量成为它的主人（聪明的读者应该能猜到，在这种情况下，该对象之前的主人就会丧失对该对象的所有权），像极了我们的现实世界，不是吗？
//! + 那为什么要引进“所有权”这个新的概念呢？请稍安勿躁，时机一旦成熟，我们就回来继续讨论这个话题。
//!```
//! pub fn intro_main(){}
//! pub fn add(i: i32, j: i32) -> i32 {i+j}
//! ```
//!

use common::outer_print;

///# 变量可变性
#[outer_print("变量可变性")]
pub fn variable_variability() {
    /// + Rust 的变量在默认情况下是不可变的。前文提到，这是 Rust 团队为我们精心设计的语言特性之一，让我们编写的代码更安全，性能也更好。当然你可以通过 mut 关键字让变量变为可变的，让设计更灵活。
    /// + 如果变量 a 不可变，那么一旦为它绑定值，就不能再修改 a。举个例子，在我们的工程目录下使用 cargo new variables 新建一个项目，叫做 variables 。
    /// ```
    ///     let x = 5;
    ///     println!("The value of x is: {}", x);
    ///     x = 6;
    ///     println!("The value of x is: {}", x);
    ///
    /// ```
    /// + 保存文件，再使用 cargo run 运行它，迎面而来的是一条错误提示：
    /// + 具体的错误原因是 cannot assign twice to immutable variable x（无法对不可变的变量进行重复赋值），因为我们想为不可变的 x 变量再次赋值。
    /// + 这种错误是为了避免无法预期的错误发生在我们的变量上：一个变量往往被多处代码所使用，其中一部分代码假定该变量的值永远不会改变，而另外一部分代码却无情的改变了这个值，在实际开发过程中，这个错误是很难被发现的，特别是在多线程编程中。
    /// + 这种规则让我们的代码变得非常清晰，只有你想让你的变量改变时，它才能改变，这样就不会造成心智上的负担，也给别人阅读代码带来便利。
    ///
    /// + 但是可变性也非常重要，否则我们就要像 ClojureScript 那样，每次要改变，就要重新生成一个对象，在拥有大量对象的场景，性能会变得非常低下，内存拷贝的成本异常的高。
    /// + 在 Rust 中，可变性很简单，只要在变量名前加一个 mut 即可, 而且这种显式的声明方式还会给后来人传达这样的信息：嗯，这个变量在后面代码部分会发生改变。
    ///
    /// + 为了让变量声明为可变,将 src/main.rs 改为以下内容：
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    /// + 选择可变还是不可变，更多的还是取决于你的使用场景，例如不可变可以带来安全性，但是丧失了灵活性和性能（如果你要改变，就要重新创建一个新的变量，这里涉及到内存对象的再分配）。
    /// + 而可变变量最大的好处就是使用上的灵活性和性能上的提升。
    /// + 例如，在使用大型数据结构或者热点代码路径（被大量频繁调用）的情形下，在同一内存位置更新实例可能比复制并返回新分配的实例要更快。使用较小的数据结构时，通常创建新的实例并以更具函数式的风格来编写程序，可能会更容易理解，所以值得以较低的性能开销来确保代码清晰。
    assert_eq!("", "") //避免文档注释报错
}

/// # 忽略未使用的变量
/// + 如果你创建了一个变量却不在任何地方使用它，Rust 通常会给你一个警告，因为这可能会是个 BUG。
/// + 但是有时创建一个不会被使用的变量是有用的，比如你正在设计原型或刚刚开始一个项目。
/// + 这时你希望告诉 Rust 不要警告未使用的变量，为此可以用下划线作为变量名的开头：
#[outer_print("忽略未使用的变量")]
pub fn ignored_unused_variable() {
    /// + 可以看到，两个变量都是只有声明，没有使用，但是编译器却独独给出了 y 未被使用的警告，充分说明了 _ 变量名前缀在这里发挥的作用。
    /// + 值得注意的是，这里编译器还很善意的给出了提示( Rust 的编译器非常强大，这里的提示只是小意思 ): 将 y 修改 _y 即可。这里就不再给出代码，留给大家手动尝试并观察下运行结果。
    /// + 更多关于 _x 的使用信息，请阅读后面的模式匹配章节。
    let _x = 5;
    let y = 10;
}

/// # 变量解构
/// let 表达式不仅仅用于变量的绑定，还能进行复杂变量的解构：从一个相对复杂的变量中，匹配出该变量的一部分内容：
///
#[outer_print("变量解构")]
pub fn variable_deconstruction() {
    let (a, mut b): (bool, bool) = (true, true);
    /// a = true,不可变; b = false，可变
    println!("a = {:?}, b = {:?}", a, b);
    println!("a = {}, b = {}", a, b);
    b = true;
    assert_eq!(a, b)
}

///# 解构式赋值
///
///
#[outer_print("解构式赋值")]
pub fn destructuring_assignments() {
    struct Struct {
        e: i32,
        f: String,
        g: i32,
    }
    let (a, b, c, d, e, f);
    (a, b) = (1, 2);//1 ,2
    [c, .., d,_] = [1, 2, 3, 4, 5];//c=1,d=4
    Struct { e, f,..
}=Struct{e: 5, f: String::from("x"), g: 1};//5
    assert_eq ! ([1, 2, 1, 4, 5], [a, b, c, d, e]);
    println! ("{} {} {} {} {}", a, b, c,d, e);
    println ! ("e:{} f:{}",  e, f);
}

///# 变量和常量之间的差异
/// ## 变量的值不能更改可能让你想起其他另一个很多语言都有的编程概念：常量(constant)。
/// ## 与不可变变量一样，常量也是绑定到一个常量名且不允许更改的值，但是常量和变量之间存在一些差异：
/// + 常量不允许使用 mut。常量不仅仅默认不可变，而且自始至终不可变，因为常量在编译完成后，已经确定它的值。
/// + 常量使用 const 关键字而不是 let 关键字来声明，并且值的类型必须标注。
#[outer_print("解构式赋值")]
pub fn diff_var_const() {
    ///下面是一个常量声明的例子，其常量名为 MAX_POINTS，值设置为 100,000。（Rust 常量的命名约定是全部字母都使用大写，并使用下划线分隔单词，另外对数字字面量可插入下划线以提高可读性）：
    const MAX_POINTS: u32 = 100_000;
    /// + 常量可以在任意作用域内声明，包括全局作用域，在声明的作用域内，常量在程序运行的整个过程中都有效。对于需要在多处代码共享一个不可变的值时非常有用，例如游戏中允许玩家赚取的最大点数或光速。
    /// + 在实际使用中，最好将程序中用到的硬编码值都声明为常量，对于代码后续的维护有莫大的帮助。如果将来需要更改硬编码的值，你也只需要在代码中更改一处即可。
    const PI: f64 = 3.14;
}

/// # 变量遮蔽(shadowing)
/// Rust 允许声明相同的变量名，在后面声明的变量会遮蔽掉前面声明的，如下所示：
#[outer_print("变量遮蔽")]
pub fn variable_shadowing() {
    /// + 这和 mut 变量的使用是不同的，第二个 let 生成了完全不同的新变量，两个变量只是恰好拥有同样的名称，涉及一次内存对象的再分配 ，而 mut 声明的变量，可以修改同一个内存地址上的值，并不会发生内存对象的再分配，性能要更好。
    /// + 变量遮蔽的用处在于，如果你在某个作用域内无需再使用之前的变量（在被遮蔽后，无法再访问到之前的同名变量），就可以重复的使用变量名字，而不用绞尽脑汁去想更多的名字。
    let x = 5;
    let x = x + 1;
    {
        let x = 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);

    // 字符串类型
    let spaces = "   ";
    // usize数值类型
    let spaces = spaces.len();
}
