use common::{inner_print, outer_print};


struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    // new是Circle的关联函数，因为它的第一个参数不是self，且new并不是关键字
    // 这种方法往往用于初始化当前结构体的实例
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x,
            y,
            radius,
        }
    }
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
    fn radius(self: &Self) -> f64 {
        self.radius
    }
    // impl Circle {} 表示为 Circle 实现方法(impl 是实现 implementation 的缩写)，这样的写法表明 impl 语句块中的一切都是跟 Circle 相关联的。
}

#[outer_print("定义方法")]
pub fn define_method() {
    let c = Circle {
        x: 1.0,
        y: 2.0,
        radius: 6.0,
    };
    println!("{:}", c.area());
    let circle = Circle::new(1f64, 2f64, 3f64);
    println!("{}", circle.radius());
    inner_print!("self、&self 和 &mut self");
    // &self 其实是 self: &Self 的简写（注意大小写）

    /// 需要注意的是，self 依然有所有权的概念：
    /// + self 表示 Rectangle 的所有权转移到该方法中，这种形式用的较少
    /// + &self 表示该方法对 Rectangle 的不可变借用
    /// + &mut self 表示可变借用

    println!("方法名跟结构体字段名相同");
    /// 用这种方式，我们可以把 Rectangle 的字段设置为私有属性，
    /// 只需把它的 new 和 width 方法设置为公开可见，那么用户就可以创建一个矩形，
    /// 同时通过访问器 rect1.width() 方法来获取矩形的宽度，
    /// 因为 width 字段是私有的，当用户访问 rect1.width 字段时，就会报错。
    /// 注意在此例中，Self 指代的就是被实现方法的结构体 Rectangle。
    println!("->运算符到哪里去了");
    /// Rust 并没有一个与 -> 等效的运算符；相反，Rust 有一个叫 自动引用和解引用的功能。方法调用是 Rust 中少数几个拥有这种行为的地方。
    /// 他是这样工作的：当使用 object.something() 调用方法时，Rust 会自动为 object 添加 &、&mut 或 * 以便使 object 与方法签名匹配。也就是说，这些代码是等价的：

    ();
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area_(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold_(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn new(w: u32, h: u32) -> Rectangle {
        Rectangle { width: w, height: h }
    }
}
#[outer_print("带有多个参数的方法")]
pub fn multipart_param_method() {


    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

#[outer_print("关联函数")]
/// 这种定义在 impl 中且没有 self 的函数被称之为关联函数：
/// 因为它没有 self，不能用 f.read() 的形式调用，因此它是一个函数而不是方法，它又在 impl 中，与结构体紧密关联，因此称为关联函数。
pub fn association_function() {
    // Rust 中有一个约定俗成的规则，使用 new 来作为构造器的名称，出于设计上的考虑，Rust 特地没有用 new 作为关键字。
    //因为是函数，所以不能用 . 的方式来调用，我们需要用 :: 来调用，例如 let sq = Rectangle::new(3, 3);。这个方法位于结构体的命名空间中：:: 语法用于关联函数和模块创建的命名空间。

}

#[outer_print("多个 impl 定义")]
/// Rust 允许我们为一个结构体定义多个 impl 块，目的是提供更多的灵活性和代码组织性，
/// 例如当方法多了后，可以把相关的方法组织在同一个 impl 块中，那么就可以形成多个 impl 块，各自完成一块儿目标：
pub fn multipart_impl() {
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
    ();

}


#[allow(unused)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self){

    }
}
#[outer_print("为枚举实现方法")]
///枚举类型之所以强大，不仅仅在于它好用、可以同一化类型，还在于，我们可以像结构体一样，为枚举实现方法：
pub fn impl_enum() {
    let m = Message::Write(String::from("hello"));
    m.call();
}