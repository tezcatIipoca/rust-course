use common::outer_print;

///元组是由多种类型组合到一起形成的，因此它是复合类型，元组的长度是固定的，元组中元素的顺序也是固定的。
#[outer_print("元组")]
pub fn tuple(){
    let tup:(i32,f64,u8)=(500, 6.4, 1);
    println!("{:?}",tup);
   // 变量 tup 被绑定了一个元组值 (500, 6.4, 1)，该元组的类型是 (i32, f64, u8)，看到没？元组是用括号将多个类型组合到一起，简单吧？

    //可以使用模式匹配或者 . 操作符来获取元组中的值。
}
#[outer_print("用模式匹配解构元组")]
pub fn match_tuple(){
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    // 上述代码首先创建一个元组，然后将其绑定到 tup 上，接着使用 let (x, y, z) = tup; 来完成一次模式匹配，
    // 因为元组是 (n1, n2, n3) 形式的，因此我们用一模一样的 (x, y, z) 形式来进行匹配，元组中对应的值会绑定到变量 x， y， z上。
    // 这就是解构：用同样的形式把一个复杂对象中的值匹配出来。
}
#[outer_print("用 . 来访问元组")]
pub fn access_tuple(){
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}

#[outer_print("元组的使用示例")]
pub fn tuple_example(){
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len(); // len() 返回字符串的长度
        (s, length)
    }
}