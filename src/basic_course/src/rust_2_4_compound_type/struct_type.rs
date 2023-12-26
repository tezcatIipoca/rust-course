use inline_c::Assert;
use common::{inner_print, outer_print};

#[outer_print("结构体语法")]
pub fn struct_syntax() {
    inner_print!("定义结构体");
    /// 一个结构体由几部分组成：
    /// 1. 通过关键字 struct 定义
    /// 2. 一个清晰明确的结构体 名称
    /// 3. 几个有名字的结构体 字段
    #[derive(Debug)]
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: i64,
    }
    //该结构体名称是 User，拥有 4 个字段，且每个字段都有对应的字段名及类型声明，例如 username 代表了用户名，是一个可变的 String 类型。

    inner_print!("创建结构体实例");
    let user1 = User {
        email: String::from("some@example.com"),
        username: String::from("some@example.com"),
        active: true,
        sign_in_count: 1,
    };
    /// 1. 初始化实例时，每个字段都需要进行初始化
    /// 2. 初始化时的字段顺序不需要和结构体定义时的顺序一致

    inner_print!("访问结构体字段");
    //需要注意的是，必须要将结构体实例声明为可变的，才能修改其中的字段，Rust 不支持将某个结构体某个字段标记为可变。
    let mut user1 = User {
        email: String::from("some@example.com"),
        username: String::from("some@example.com"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = "another_email@example.com".to_string();

    inner_print!("简化结构体创建");
    //下面的函数类似一个构建函数，返回了 User 结构体的实例：
    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }
    //如上所示，当函数参数和结构体字段同名时，可以直接使用缩略的方式进行初始化，跟 TypeScript 中一模一样。

    inner_print!("结构体更新语法");
    //根据已有的结构体实例，创建新的结构体实例，例如根据已有的 user1 实例来构建 user2：
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    //老话重提，如果你从 TypeScript 过来，肯定觉得啰嗦爆了：竟然手动把 user1 的三个字段逐个赋值给 user2，好在 Rust 为我们提供了 结构体更新语法：
    let user3 = User {
        email: String::from("another@example.com"),
        ..user2
    };
    ///.. 语法表明凡是我们没有显式声明的字段，全部从 user1 中自动获取。需要注意的是 ..user1 必须在结构体的尾部使用。

    /// 结构体更新语法跟赋值语句 = 非常相像，因此在上面代码中，user1 的部分字段所有权被转移到 user2 中：username 字段发生了所有权转移，作为结果，user1 无法再被使用。
    /// 明明有三个字段进行了自动赋值，为何只有 username 发生了所有权转移？
    /// 我们提到了 Copy 特征：实现了 Copy 特征的类型无需所有权转移，可以直接在赋值时进行 数据拷贝，其中 bool 和 u64 类型就实现了 Copy 特征，因此 active 和 sign_in_count 字段在赋值给 user2 时，仅仅发生了拷贝，而不是所有权转移。
    /// 值得注意的是：username 所有权被转移给了 user2，导致了 user1 无法再被使用，但是并不代表 user1 内部的其它字段不能被继续使用，例如：
    println!("{}", user2.active);
    // println!("{:?}",user2); //borrow of partially moved value: `user2`
    inner_print!("结构体整体所有权和部分字段所有权");
    #[derive(Debug)]
    struct U {
        uname: String,
        pass: String,
    }
    let u1 = U {
        uname: "uname1".to_string(),
        pass: "123abc".to_string(),
    };
    println!("{:?},{},{}", u1,u1.uname,u1.pass);
    let u2=U {
        uname: "uname2".to_string(),
        pass: u1.pass
    };
    println!("{}", u1.uname);
    println!("{:?}", u2);
    // println!("{}", u1.pass); //------- value moved here
    // println!("{:?}", u1);   //borrow of partially moved value: `u1`

}

#[outer_print("结构体内存排列")]
pub fn struct_mem() {
    #[derive(Debug)]
    struct File {
        name: String,
        data: Vec<u8>,
    }

    fn main() {
        let f1 = File {
            name: String::from("f1.txt"),
            data: Vec::new(),
        };

        let f1_name = &f1.name;
        let f1_length = &f1.data.len();

        println!("{:?}", f1);
        println!("{} is {} bytes long", f1_name, f1_length);
    }
    main();
    //上面定义的 File 结构体在内存中的排列如下图所示：
    // https://course.rs/basic/compound-type/struct.html#%E7%AE%80%E5%8C%96%E7%BB%93%E6%9E%84%E4%BD%93%E5%88%9B%E5%BB%BA
    // 从图中可以清晰地看出 File 结构体两个字段 name 和 data 分别拥有底层两个 [u8] 数组的所有权(String 类型的底层也是 [u8] 数组)，
    // 通过 ptr 指针指向底层数组的内存地址，这里你可以把 ptr 指针理解为 Rust 中的引用类型。
    ///该图片也侧面印证了：把结构体中具有所有权的字段转移出去后，将无法再访问该字段，但是可以正常访问其它的字段。
    ();
}

#[outer_print("元组结构体")]
pub fn tuple_struct() {
    //结构体必须要有名称，但是结构体的字段可以没有名称，这种结构体长得很像元组，因此被称为元组结构体，例如：
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

#[outer_print("单元结构体")]
pub fn unit_like_struct() {
    ///如果你定义一个类型，但是不关心该类型的内容, 只关心它的行为时，就可以使用 单元结构体：
    struct AlwaysEqual;

    let subject = AlwaysEqual;
    // 我们不关心 AlwaysEqual 的字段数据，只关心它的行为，因此将它声明为单元结构体，然后再为它实现某个特征
    impl AlwaysEqual {
        fn new() -> Self {
            AlwaysEqual {}
        }
        fn do_something(&self) {
            println!("Doing something with a unit struct.");
        }
    }
}

#[outer_print("结构体所有权")]
pub fn struct_ownership() {
    //  在之前的 User 结构体的定义中，有一处细节：我们使用了自身拥有所有权的 String 类型而不是基于引用的 &str 字符串切片类型。
    // 这是一个有意而为之的选择：因为我们想要这个结构体拥有它所有的数据，而不是从其它地方借用数据。
    ///你也可以让 User 结构体从其它对象借用数据，不过这么做，就需要引入生命周期(lifetimes)这个新概念（也是一个复杂的概念），简而言之，生命周期能确保结构体的作用范围要比它所借用的数据的作用范围要小。
    //总之，如果你想在结构体中使用一个引用，就必须加上生命周期，否则就会报错：
    /*
    struct User {
        username: &str,
        email: &str,
        sign_in_count: u64,
        active: bool,
    }

    fn main() {
        let user1 = User {
            email: "someone@example.com",
            username: "someusername123",
            active: true,
            sign_in_count: 1,
        };
    }
     */
    //编译器会抱怨它需要生命周期标识符：
    ();
}

#[outer_print("使用 #[derive(Debug)] 来打印结构体的信息")]
pub fn struct_print() {
    //在前面的代码中我们使用 #[derive(Debug)] 对结构体进行了标记，这样才能使用 println!("{:?}", s); 的方式对其进行打印输出，如果不加，看看会发生什么:
    // struct Rectangle {
    //     width: u32,
    //     height: u32,
    // }
    //
    // fn main() {
    //     let rect1 = Rectangle {
    //         width: 30,
    //         height: 50,
    //     };
    //
    //     println!("rect1 is {}", rect1);
    // }
    /// 首先可以观察到，上面使用了 {} 而不是之前的 {:?}，运行后报错：
    /// error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
    ///提示我们结构体 Rectangle 没有实现 Display 特征，这是因为如果我们使用 {} 来格式化输出，那对应的类型就必须实现 Display 特征，以前学习的基本类型，都默认实现了该特征:
    // 上面代码不会报错，那么结构体为什么不默认实现 Display 特征呢？原因在于结构体较为复杂，例如考虑以下问题：你想要逗号对字段进行分割吗？需要括号吗？加在什么地方？所有的字段都应该显示？
    // 类似的还有很多，由于这种复杂性，Rust 不希望猜测我们想要的是什么，而是把选择权交给我们自己来实现：如果要用 {} 的方式打印结构体，那就自己实现 Display 特征。
    ///上面提示我们使用 {:?} 来试试，这个方式我们在本文的前面也见过，下面来试试,可是依然无情报错了:
    /// error[E0277]: `Rectangle` doesn't implement `Debug`
    // 让我们实现 Debug 特征，Oh No，就是不想实现 Display 特征，才用的 {:?}，怎么又要实现 Debug，但是仔细看，提示中有一行： add #[derive(Debug)] to Rectangle， 哦？这不就是我们前文一直在使用的吗？
    //首先，Rust 默认不会为我们实现 Debug，为了实现，有两种方式可以选择：
    /// + 手动实现
    /// + 使用 derive 派生实现
    /// 当结构体较大时，我们可能希望能够有更好的输出表现，此时可以使用 {:#?} 来替代 {:?}，输出如下:

    /// 还有一个简单的输出 debug 信息的方法，那就是使用 dbg! 宏，它会拿走表达式的所有权，然后打印出相应的文件名、行号等 debug 信息，当然还有我们需要的表达式的求值结果。除此之外，它最终还会把表达式值的所有权返回！
    /// dbg! 输出到标准错误输出 stderr，而 println! 输出到标准输出 stdout。
    ()
}
