use std::io;
use common::{inner_print, outer_print};

#[outer_print("创建数组")]
pub fn create_array(){
    /// 数组的具体定义很简单：将多个类型相同的元素依次组合在一起，就是一个数组。结合上面的内容，可以得出数组的三要素：
    /// + 长度固定
    /// + 元素必须有相同的类型
    /// + 依次线性排列
    //我们这里说的数组是 Rust 的基本类型，是固定长度的，这点与其他编程语言不同，其它编程语言的数组往往是可变长度的，与 Rust 中的动态数组 Vector 类似

    inner_print!("创建数组");
    let a = [1, 2, 3, 4, 5];
    //  数组语法跟 JavaScript 很像，也跟大多数编程语言很像。由于它的元素类型大小固定，且长度也是固定，因此数组 array 是存储在栈上，性能也会非常优秀。
    // 与此对应，动态数组 Vector 是存储在堆上，因此长度可以动态改变。当你不确定是使用数组还是动态数组时，那就应该使用后者，具体见动态数组 Vector。
    //举个例子，在需要知道一年中各个月份名称的程序中，你很可能希望使用的是数组而不是动态数组。因为月份是固定的，它总是只包含 12 个元素：
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    //在一些时候，还需要为数组声明类型，如下所示：
    let a:[i32;5]=[1, 2, 3, 4, 5];
    ///这里，数组类型是通过方括号语法声明，i32 是元素类型，分号后面的数字 5 是数组长度，数组类型也从侧面说明了数组的元素类型要统一，长度要固定。
    /// 还可以使用下面的语法初始化一个某个值重复出现 N 次的数组：
    let a=[3;5];
    //等价于 let a = [3, 3, 3, 3, 3];
    println!("{:?}",a);

}

#[outer_print("访问数组")]
pub fn access_array(){
    //因为数组是连续存放元素的，因此可以通过索引的方式来访问存放其中的元素：
    let a = [9, 8, 7, 6, 5];

    let first = a[0]; // 获取a数组第一个元素
    let second = a[1]; // 获取第二个元素
    inner_print!("越界访问");
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();
    // 读取控制台的输出
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    //当你尝试使用索引访问元素时，Rust 将检查你指定的索引是否小于数组长度。如果索引大于或等于数组长度，Rust 会出现 panic。这种检查只能在运行时进行，比如在上面这种情况下，编译器无法在编译期知道用户运行代码时将输入什么值。

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
    inner_print!("数组元素为非基础类型");
    let array = [String::from("rust is good!"); 8];
    println!("{:#?}", array);
    //然后你会惊喜的得到编译错误。
    /// 而基本类型在Rust中赋值是以Copy的形式，这时候你就懂了吧，let array=[3;5]底层就是不断的Copy出来的，但很可惜复杂类型都没有深拷贝，只能一个个创建。
    // 正确的写法，应该调用std::array::from_fn
    let array: [String; 8] = std::array::from_fn(|_i| String::from("rust is good!"));
    println!("{:#?}", array);


}

#[outer_print("数据切片")]
pub fn array_slice(){
    //在之前的章节，我们有讲到 切片 这个概念，它允许你引用集合中的部分连续片段，而不是整个集合，对于数组也是，数组切片允许我们引用数组的一部分：
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let slice: &[i32] = &a[1..3];

    assert_eq!(slice, &[2, 3]);
    /// 切片的长度可以与数组不同，并不是固定的，而是取决于你使用时指定的起始和结束位置
    /// 创建切片的代价非常小，因为切片只是针对底层数组的一个引用
    /// 切片类型[T]拥有不固定的大小，而切片引用类型&[T]则具有固定的大小，因为 Rust 很多时候都需要固定大小数据类型，因此&[T]更有用,&str字符串切片也同理
}