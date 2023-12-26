use std::ops::Add;
use inline_c::predicates::predicate::le;
use common::{inner_print, outer_print};

#[outer_print("复合类型")]
pub fn compound_type() {
    #![allow(unused_variables)]
    type File = String;

    fn open(f: &mut File) -> bool {
        true
    }
    fn close(f: &mut File) -> bool {
        true
    }

    #[allow(dead_code)]
    fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
        unimplemented!()
    }

    fn main() {
        let mut f1 = File::from("f1.txt");
        open(&mut f1);
        //read(&mut f1, &mut vec![]);
        close(&mut f1);
    }
}

#[outer_print("字符串")]
pub fn string() {
    // let my_name="Pascal"; //错误,类型是字符串切片不是字符串
    let my_name: String = String::from("Pascal");
    greet(my_name);
    fn greet(name: String) {
        println!("Hello, {}!", name);
    }
}

#[outer_print("切片")]
/// 切片并不是 Rust 独有的概念，在 Go 语言中就非常流行，它允许你引用集合中部分连续的元素序列，而不是引用整个集合。
/// 对于字符串而言，切片就是对 String 类型中某一部分的引用，它看起来像这样：
pub fn slice() {
    let s = String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..];
    println!("{},{}", hello, world);
    //hello 没有引用整个 String s，而是引用了 s 的一部分内容，通过 [0..5] 的方式来指定。
    /// 这就是创建切片的语法，使用方括号包括的一个序列：[开始索引..终止索引]，其中开始索引是切片中第一个元素的索引位置，而终止索引是最后一个元素后面的索引位置，也就是这是一个 右半开区间。
    /// 在切片数据结构内部会保存开始的位置和切片的长度，其中长度是通过 终止索引 - 开始索引 的方式计算得来的。
    //对于 let world = &s[6..11]; 来说，world 是一个切片，该切片的指针指向 s 的第 7 个字节(索引从 0 开始, 6 是第 7 个字节)，且该切片的长度是 5 个字节。

    ///在使用 Rust 的 .. range 序列语法时，如果你想从索引 0 开始，可以使用如下的方式，这两个是等效的：
    let s = String::from("hello");
    let slice = &s[0..2];
    let slice = &s[..2];
    println!("{}", slice);

    // 同样的，如果你的切片想要包含 String 的最后一个字节，则可以这样使用：
    let s = String::from("hello");
    let len = s.len();
    let slice = &s[4..len];
    let slice = &s[4..];
    println!("{}", slice);

    //你也可以截取完整的 String 切片：
    let s = String::from("hello");
    let len = s.len();
    let slice = &s[0..len];
    let slice = &s[..];
    println!("{}", slice);

    /// 在对字符串使用切片语法时需要格外小心，切片的索引必须落在字符之间的边界位置，也就是 UTF-8 字符的边界，例如中文在 UTF-8 中占用三个字节，下面的代码就会崩溃：
    /// 字符串切片的类型标识是 &str，因此我们可以这样声明一个函数，输入 String 类型，返回它的切片: fn first_word(s: &String) -> &str 。
    // 有了切片就可以写出这样的代码：
    inner_print!("其他切片");
    //因为切片是对集合的部分引用，因此不仅仅字符串有切片，其它集合类型也有，例如数组：
    let a = [1, 2, 3, 4, 5];
    let slice = &a[..3];
    println!("{:?}", slice);
}

#[outer_print("字符串字面量是切片")]
pub fn str_literal() {
    //之前提到过字符串字面量,但是没有提到它的类型：
    let s = "Hello, world!";
    //实际上，s 的类型是 &str，因此你也可以这样声明：
    let s: &str = "Hello, world!";
    ///该切片指向了程序可执行文件中的某个点，这也是为什么字符串字面量是不可变的，因为 &str 是一个不可变引用。
    ();
}

#[outer_print("什么是字符串?")]
/// 顾名思义，字符串是由字符组成的连续集合
/// Rust 中的字符是 Unicode 类型，因此每个字符占据 4 个字节内存空间，但是在字符串中不一样，字符串是 UTF-8 编码，也就是字符串中的字符所占的字节数是变化的(1 - 4)，这样有助于大幅降低字符串所占用的内存空间。
pub fn define_str() {
    /// Rust 在语言级别，只有一种字符串类型： str，它通常是以引用类型出现 &str，也就是上文提到的字符串切片。
    /// 虽然语言级别只有上述的 str 类型，但是在标准库里，还有多种不同用途的字符串类型，其中使用最广的即是 String 类型。

    /// str 类型是硬编码进可执行文件，也无法被修改，但是 String 则是一个可增长、可改变且具有所有权的 UTF-8 编码字符串，
    /// 当 Rust 用户提到字符串时，往往指的就是 String 类型和 &str 字符串切片类型，这两个类型都是 UTF-8 编码。

    /// 除了 String 类型的字符串，Rust 的标准库还提供了其他类型的字符串，例如 OsString， OsStr， CsString 和 CsStr 等，
    /// 注意到这些名字都以 String 或者 Str 结尾了吗？它们分别对应的是具有所有权和被借用的变量。

    /// str：它是一个不可变的、原始的字符串 slice，不拥有其内部数据的所有权。str 通常是通过引用 (&str) 来使用的，它指向一个固定大小的、UTF-8 编码的字符序列，该序列可以位于堆栈或堆中。
    /// String：它是一个可变的、 growable 的字符串类型，拥有其内部数据的所有权。String 存储在堆上，并且可以动态地改变其内容和长度。
    ();
}

#[outer_print("String 与 &str 的转换")]
pub fn convert_str() {
    //将 str转换成String
    let s1 = String::from("hello,world");
    let s1 = "hello,world".to_string();

    //将 String 类型转为 &str 类型呢

    let s = String::from("hello,world!");
    say_hello(&s);
    say_hello(&s[..]);
    say_hello(s.as_str());
    fn say_hello(s: &str) {
        println!("{}", s);
    }
}

#[outer_print("字符串索引")]
pub fn str_index() {
    /// 在其它语言中，使用索引的方式访问字符串的某个字符或者子串是很正常的行为，但是在 Rust 中就会报错：
    /// let s1 = String::from("hello");
    /// let h = s1[0];

    inner_print!("深入字符串内部");
    // 字符串的底层的数据存储格式实际上是[ u8 ]，一个字节数组
    // 对于 let hello = String::from("Hola");
    // 这行代码来说，Hola 的长度是 4 个字节，因为 "Hola" 中的每个字母在 UTF-8 编码中仅占用 1 个字节，但是对于下面的代码呢？
    let hello = String::from("中国字");
    //  但是实际上是 9 个字节的长度，因为大部分常用汉字在 UTF-8 中的长度是 3 个字节，
    // 因此这种情况下对 hello 进行索引，访问 &hello[0] 没有任何意义，因为你取不到 中 这个字符，而是取到了这个字符三个字节中的第一个字节

    inner_print!("字符串的不同表现形式");
}

#[outer_print("字符串切片")]
pub fn str_slice() {
    //前文提到过，字符串切片是非常危险的操作，因为切片的索引是通过字节来进行，但是字符串又是 UTF-8 编码，因此你无法保证索引的字节刚好落在字符的边界上，例如：
    ///let hello = "中国人";
    /// let s = &hello[0..2];

    //这里提示的很清楚，我们索引的字节落在了 中 字符的内部，这种返回没有任何意义。
    // 因此在通过索引区间来访问字符串时，需要格外的小心，一不注意，就会导致你程序的崩溃！
    ();
}

#[outer_print("操作字符串")]
pub fn manipulating_str() {
    //由于 String 是可变字符串，下面介绍 Rust 字符串的修改，添加，删除等常用方法：

    inner_print!("追加 (Push)");
    // 在字符串尾部可以使用 push() 方法追加字符 char，也可以使用 push_str() 方法追加字符串字面量。
    /// 这两个方法都是在原有的字符串上追加，并不会返回新的字符串。
    /// 由于字符串追加操作要修改原来的字符串，则该字符串必须是可变的，即字符串变量必须由 mut 关键字修饰。
    let mut s = String::from("hello ");
    s.push_str("rust");
    println!("追加字符串 push_str() -> {}", s);
    s.push('字');
    println!("追加字符 push() -> {}", s);

    inner_print!("插入 (Insert)");
    //可以使用 insert() 方法插入单个字符 char，也可以使用 insert_str() 方法插入字符串字面量，与 push() 方法不同，这俩方法需要传入两个参数，第一个参数是字符（串）插入位置的索引，第二个参数是要插入的字符（串），索引从 0 开始计数，如果越界则会发生错误。
    ///由于字符串插入操作要修改原来的字符串，则该字符串必须是可变的，即字符串变量必须由 mut 关键字修饰。
    let mut s = String::from("Hello rust!");
    s.insert(5, ',');
    println!("插入字符 insert() -> {}", s);
    s.insert_str(6, " I like");
    println!("插入字符串 insert_str() -> {}", s);

    inner_print!("替换 (Replace)");
    ///replace
    //  该方法可适用于 String 和 &str 类型。replace() 方法接收两个参数，第一个参数是要被替换的字符串，第二个参数是新的字符串。该方法会替换所有匹配到的字符串。
    /// 该方法是返回一个新的字符串，而不是操作原来的字符串。
    let string_replace = String::from("I like rust. Learning rust is my favorite!");
    let new_string_replace = string_replace.replace("rust", "RUST");
    dbg!(new_string_replace);
    ///replacen
    //  该方法可适用于 String 和 &str 类型。replacen() 方法接收三个参数，前两个参数与 replace() 方法一样，第三个参数则表示替换的个数。
    /// 该方法是返回一个新的字符串，而不是操作原来的字符串。
    let string_replace = "I like rust. Learning rust is my favorite!";
    let new_string_replacen = string_replace.replacen("rust", "RUST", 1);
    dbg!(new_string_replacen);
    ///replace_range
    //  该方法仅适用于 String 类型。replace_range 接收两个参数，第一个参数是要替换字符串的范围（Range），第二个参数是新的字符串。
    /// 该方法是直接操作原来的字符串，不会返回新的字符串。该方法需要使用 mut 关键字修饰。

    inner_print!("删除 (Delete)");
    //与字符串删除相关的方法有 4 个，他们分别是 pop()，remove()，truncate()，clear()。这四个方法仅适用于 String 类型。
    /// pop —— 删除并返回字符串的最后一个字符
    /// 该方法是直接操作原来的字符串。但是存在返回值，其返回值是一个 Option 类型，如果字符串为空，则返回 None。 示例代码如下：
    let mut string_pop = String::from("rust pop 中文!");
    let p1 = string_pop.pop();
    let p2 = string_pop.pop();
    dbg!(p1);
    dbg!(p2);
    dbg!(string_pop);
    /// remove —— 删除并返回字符串中指定位置的字符
    /// 该方法是直接操作原来的字符串。但是存在返回值，其返回值是删除位置的字符串，只接收一个参数，表示该字符起始索引位置。remove() 方法是按照字节来处理字符串的，如果参数所给的位置不是合法的字符边界，则会发生错误。
    let mut string_remove = String::from("测试remove方法");
    println!(
        "string_remove 占 {} 个字节",
        std::mem::size_of_val(string_remove.as_str())
    );
    // 删除第一个汉字
    string_remove.remove(0);
    // 下面代码会发生错误
    // string_remove.remove(1);
    // 删除第二个汉字
    string_remove.remove(0);
    dbg!(string_remove);
    ///truncate —— 删除字符串中从指定位置开始到结尾的全部字符
    ///该方法是直接操作原来的字符串。无返回值。该方法 truncate() 方法是按照字节来处理字符串的，如果参数所给的位置不是合法的字符边界，则会发生错误。
    let mut string_truncate = String::from("测试truncate");
    string_truncate.truncate(3);
    dbg!(string_truncate);
    /// clear —— 清空字符串truncate —— 删除字符串中从指定位置开始到结尾的全部字符
    /// 该方法是直接操作原来的字符串。调用后，删除字符串中的所有字符，相当于 truncate() 方法参数为 0 的时候。
    let mut string_clear = String::from("string clear");
    string_clear.clear();
    dbg!(string_clear);

    inner_print!("连接 (Concatenate)");
    /// 使用 + 或者 += 连接字符串
    /// 使用 + 或者 += 连接字符串，要求右边的参数必须为字符串的切片引用（Slice）类型。
    /// 其实当调用 + 的操作符时，相当于调用了 std::string 标准库中的 add() 方法，这里 add() 方法的第二个参数是一个引用的类型。
    /// 因此我们在使用 +， 必须传递切片引用类型。不能直接传递 String 类型。+ 是返回一个新的字符串，所以变量声明可以不需要 mut 关键字修饰。
    let string_append = String::from("hello");
    let string_rs = String::from("rust");
    let result = string_append + &*string_rs;
    let mut result = result + "!";
    result += "!!!";
    result = result.add("final");
    println!("连接字符串 + -> result:{}  ", result);
    /// 使用 format! 连接字符串
    /// format! 这种方式适用于 String 和 &str 。format! 的用法与 print! 的用法类似，详见格式化输出。
    let s1 = "hello";
    let s2 = String::from("rust");
    let s = format!("{} {}!", s1, s2);
    println!("{}", s);
}

#[outer_print("字符串转义")]
pub fn str_escape() {
    //我们可以通过转义的方式 \ 输出 ASCII 和 Unicode 字符。
    /// 通过 \ + 字符的十六进制表示，转义输出一个字符
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    /// \u 可以输出一个 unicode 字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    // 换行了也会保持之前的字符串格式
    /// 使用\忽略换行符
    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);

    ///当然，在某些情况下，可能你会希望保持字符串的原样，不要转义：
    println!("{}", "hello \\x52\\x75\\x73\\x74");
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 如果字符串包含双引号，可以在开头和结尾加 #
    let quotes = r###"And then I said: "There is no escape!""###;
    println!("{}", quotes);

    // 如果还是有歧义，可以继续增加，没有限制
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);
}

#[outer_print("操作 UTF-8 字符串")]
pub fn utf8_str() {
    inner_print!("字符");
    //如果你想要以 Unicode 字符的方式遍历字符串，最好的办法是使用 chars 方法，例如：
    for c in "中国人".chars() {
        println!("{}", c);
    }

    inner_print!("字节");
    // 这种方式是返回字符串的底层字节数组表现形式：
    for b in "中国人".bytes() {
        println!("{}", b);
    }

    inner_print!("获取子串");
    //想要准确的从 UTF-8 字符串中获取子串是较为复杂的事情，例如想要从 hello中国人这种变长的字符串中取出某一个子串，使用标准库你是做不到的。 你需要在 crates.io 上搜索 utf8 来寻找想要的功能。

}

#[outer_print("字符串深度剖析")]
pub fn str_analyze() {
    ///那么问题来了，为啥 String 可变，而字符串字面值 str 却不可以？
    // 就字符串字面值来说，我们在编译时就知道其内容，最终字面值文本被直接硬编码进可执行文件中，这使得字符串字面值快速且高效，这主要得益于字符串字面值的不可变性。
    // 不幸的是，我们不能为了获得这种性能，而把每一个在编译时大小未知的文本都放进内存中（你也做不到！），因为有的字符串是在程序运行得过程中动态生成的。

    // 对于 String 类型，为了支持一个可变、可增长的文本片段，需要在堆上分配一块在编译时未知大小的内存来存放内容，这些都是在程序运行时完成的：
    /// 首先向操作系统请求内存来存放 String 对象
    /// 在使用完成后，将内存释放，归还给操作系统
    //  其中第一部分由 String::from 完成，它创建了一个全新的 String。
    // 在有垃圾回收 GC 的语言中，GC 来负责标记并清除这些不再使用的内存对象，这个过程都是自动完成，无需开发者关心，非常简单好用；但是在无 GC 的语言中，需要开发者手动去释放这些内存对象，就像创建对象需要通过编写代码来完成一样，未能正确释放对象造成的后果简直不可估量。
    //  rust则变量在离开作用域后，就自动释放其占用的内存：
    {
        let s = String::from("hello"); // 从此处起，s 是有效的

        // 使用 s
    }                                  // 此作用域已结束，
    // s 不再有效，内存被释放
    //与其它系统编程语言的 free 函数相同，Rust 也提供了一个释放内存的函数： drop，但是不同的是，其它语言要手动调用 free 来释放每一个变量占用的内存，而 Rust 则在变量离开作用域时，自动调用 drop 函数: 上面代码中，Rust 在结尾的 } 处自动调用 drop。

}