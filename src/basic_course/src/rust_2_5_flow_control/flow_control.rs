use common::outer_print;

#[outer_print("使用 if 来做分支控制")]
pub fn if_(){
    // if condition == true {
    //      A...
    // } else {
    //      B...
    // }
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
    // if 语句块是表达式，这里我们使用 if 表达式的返回值来给 number 进行赋值：number 的值是 5
    // 用 if 来赋值时，要保证每个分支返回的类型一样(事实上，这种说法不完全准确，见这里)，此处返回的 5 和 6 就是同一个类型，如果返回类型不一致就会报错
}

#[outer_print("")]
pub fn else_if(){
        let n = 6;

        if n % 4 == 0 {
            println!("number is divisible by 4");
        } else if n % 3 == 0 {
            println!("number is divisible by 3");
        } else if n % 2 == 0 {
            println!("number is divisible by 2");
        } else {
            println!("number is not divisible by 4, 3, or 2");
        }
}


#[outer_print("for循环")]
pub fn for_(){
    for i in 1..6 {
        println!("{}",i)
    };
    ///以上代码循环输出一个从 1 到 5 的序列，简单粗暴，核心就在于 for 和 in 的联动，语义表达如下：
    /*
        for 元素 in 集合 {
          // 使用元素干一些你懂我不懂的事情
          }
     */
    /// 注意，使用 for 时我们往往使用集合的引用形式，除非你不想在后面的代码中继续使用该集合（比如我们这里使用了 container 的引用）。
    /// 如果不使用引用的话，所有权会被转移（move）到 for 语句块中，后面就无法再使用这个集合了)：
    /*
        for item in &container {
          // ...
        }
     */
    ///对于实现了 copy 特征的数组(例如 [i32; 10] )而言， for item in arr 并不会把 arr 的所有权转移，而是直接对其进行了拷贝，因此循环之后仍然可以使用 arr 。

    ///如果想在循环中，修改该元素，可以使用 mut 关键字：
    /// 使用方法	                        等价使用方式	                                    所有权
    /// for item in collection	        for item in IntoIterator::into_iter(collection)	转移所有权
    /// for item in &collection	        for item in collection.iter()	                不可变借用
    /// for item in &mut collection	    for item in collection.iter_mut()	            可变借用

    //如果想在循环中获取元素的索引：
    let a = [4, 3, 2, 1];
    for (k,v) in a.iter().enumerate()  {
        println!("第{}个元素是{}", k+1,v);
    }
    //有同学可能会想到，如果我们想用 for 循环控制某个过程执行 10 次，但是又不想单独声明一个变量来控制这个流程，该怎么写？
    for _ in 0..10 {

    }
    /// 性能：第一种使用方式中 collection[index] 的索引访问，会因为边界检查(Bounds Checking)导致运行时的性能损耗 —— Rust 会检查并确认 index 是否落在集合内，但是第二种直接迭代的方式就不会触发这种检查，因为编译器会在编译时就完成分析并证明这种访问是合法的
    /// 安全：第一种方式里对 collection 的索引访问是非连续的，存在一定可能性在两次访问之间，collection 发生了变化，导致脏数据产生。而第二种直接迭代的方式是连续访问，因此不存在这种风险( 由于所有权限制，在访问过程中，数据并不会发生变化)。
    ()
}

#[outer_print("continue")]
pub fn continue_(){
    for i in 1..4 {
        if i == 2 {
            continue;
        }
        println!("{}", i);
    }
}

#[outer_print("break")]
pub fn break_(){
    for i in 1..4 {
        if i == 2 {
            break;
        }
        println!("{}", i);
    }
}
#[outer_print("while循环")]
pub fn while_(){
    //如果你需要一个条件来循环，当该条件为 true 时，继续循环，条件为 false，跳出循环，那么 while 就非常适用：
    let mut n=0;
    while n<5 {
        println!("{}!",n);
        n=n+1;
    }
    println!("out");
}
#[outer_print("loop循环")]
pub fn loop_(){
    //对于循环而言，loop 循环毋庸置疑，是适用面最高的，它可以适用于所有循环场景（虽然能用，但是在很多场景下， for 和 while 才是最优选择），因为 loop 就是一个简单的无限循环，你可以在内部实现逻辑通过 break 关键字来控制循环何时结束。
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
    ///  break 可以单独使用，也可以带一个返回值，有些类似 return
    ///  loop 是一个表达式，因此可以返回一个值
    ()
}