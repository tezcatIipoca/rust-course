use common::{inner_print, outer_print};

#[outer_print("枚举值")]
pub fn enum_value() {
    inner_print!("枚举");
    ///枚举(enum 或 enumeration)允许你通过列举可能的成员来定义一个枚举类型，例如扑克牌花色：
    #[derive(Debug)]
    enum PokerSuit {
        Clubs,
        Spades,
        Diamonds,
        Hearts,
    }
    ;
    ///总而言之： 枚举类型是一个类型，它会包含所有可能的枚举成员, 而枚举值是该类型中的具体某个成员的实例。

    inner_print!("枚举值");
    //现在来创建 PokerSuit 枚举类型的两个成员实例：
    let heart = PokerSuit::Hearts;
    let diamond = PokerSuit::Diamonds;

    print_suit(heart);
    print_suit(diamond);

    fn print_suit(card: PokerSuit) {
        // 需要在定义 enum PokerSuit 的上面添加上 #[derive(Debug)]，否则会报 card 没有实现 Debug
        println!("{:?}", card);
    }
    //目前来说，枚举值还不能带有值，因此先用结构体来实现：
    struct PokerCard {
        suit: PokerSuit,
        value: u8,
    }
    let c1 = PokerCard {
        suit: PokerSuit::Clubs,
        value: 1,
    };
    let c2 = PokerCard {
        suit: PokerSuit::Diamonds,
        value: 12,
    };
    //这段代码很好的完成了它的使命，通过结构体 PokerCard 来代表一张牌，结构体的 suit 字段表示牌的花色，类型是 PokerSuit 枚举类型，value 字段代表扑克牌的数值。

    //可以吗？可以！好吗？说实话，不咋地，因为还有简洁得多的方式来实现：
    {
        enum PokerCard {
            Clubs(u8),
            Spades(u8),
            Diamonds(u8),
            Hearts(u8),
        }
        let c1 = PokerCard::Spades(5);
        let c2 = PokerCard::Diamonds(13);
    }

    //不仅如此，同一个枚举类型下的不同成员还能持有不同的数据类型，例如让某些花色打印 1-13 的字样，另外的花色打印上 A-K 的字样：
    {
        enum PokerCard {
            Clubs(u8),
            Spades(u8),
            Diamonds(char),
            Hearts(char),
        }

        let c1 = PokerCard::Spades(5);
        let c2 = PokerCard::Diamonds('A');
    }

    //再来看一个来自标准库中的例子：
    {
        struct Ipv4Addr {
            // --snip--
        }

        struct Ipv6Addr {
            // --snip--
        }

        enum IpAddr {
            V4(Ipv4Addr),
            V6(Ipv6Addr),
        }
    }
    ///从这些例子可以看出，任何类型的数据都可以放入枚举成员中: 例如字符串、数值、结构体甚至另一个枚举。
    {
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        let m1 = Message::Quit;
        let m2 = Message::Move { x: 1, y: 1 };
        let m3 = Message::ChangeColor(255, 255, 0);
        // 该枚举类型代表一条消息，它包含四个不同的成员：
        //  Quit 没有任何关联数据
        // Move 包含一个匿名结构体
        // Write 包含一个 String 字符串
        // ChangeColor 包含三个 i32
    }
}

#[outer_print("同一化类型")]
pub fn uniform_type() {
    //例如我们有一个 WEB 服务，需要接受用户的长连接，假设连接有两种：TcpStream 和 TlsStream，但是我们希望对这两个连接的处理流程相同，也就是用同一个函数来处理这两个连接，代码如下：

    // fn new (stream: TcpStream) {
    //     let mut s = stream;
    //     if tls {
    //         s = negotiate_tls(stream)
    //     }
    //
    //     // websocket是一个WebSocket<TcpStream>或者
    //     //   WebSocket<native_tls::TlsStream<TcpStream>>类型
    //     websocket = WebSocket::from_raw_socket(
    //         stream, ......)
    // }


    //此时，枚举类型就能帮上大忙：

    // enum Websocket {
    //     Tcp(Websocket<TcpStream>),
    //     Tls(Websocket<native_tls::TlsStream<TcpStream>>),
    // }
}

#[outer_print("Option 枚举用于处理空值")]
pub fn option_enum() {
    //  在其它编程语言中，往往都有一个 null 关键字，该关键字用于表明一个变量当前的值为空（不是零值，例如整型的零值是 0），也就是不存在值。
    // 当你对这些 null 进行操作时，例如调用一个方法，就会直接抛出null 异常，导致程序的崩溃，因此我们在编程时需要格外的小心去处理这些 null 空值。
    /// 有鉴于此，Rust 吸取了众多教训，决定抛弃 null，而改为使用 Option 枚举变量来表述这种结果。
    /// Option 枚举包含两个成员，一个成员表示含有值：Some(T), 另一个表示没有值：None，定义如下：
    enum Option<T> {
        Some(T),
        None,
    }
    ///其中 T 是泛型参数，Some(T)表示该枚举成员的数据类型是 T，换句话说，Some 可以包含任何类型的数据。
    /// Option<T> 枚举是如此有用以至于它被包含在了 prelude，你不需要将其显式引入作用域。另外，它的成员 Some 和 None 也是如此，无需使用 Option:: 前缀就可直接使用 Some 和 None。
    /// 总之，不能因为 Some(T) 和 None 中没有 Option:: 的身影，就否认它们是 Option 下的卧龙凤雏。
    let some_number = Some(5);
    let some_string = Some("a string");
    // let absent_number: Option<i32> = None;
    // 如果使用 None 而不是 Some，需要告诉 Rust Option<T> 是什么类型的，因为编译器只通过 None 值无法推断出 Some 成员保存的值的类型。
    // 当有一个 Some 值时，我们就知道存在一个值，而这个值保存在 Some 中。当有个 None 值时，在某种意义上，它跟空值具有相同的意义：并没有一个有效的值。那么，Option<T> 为什么就比空值要好呢？
    // 简而言之，因为 Option<T> 和 T（这里 T 可以是任何类型）是不同的类型，例如，这段代码不能编译，因为它尝试将 Option<i8>(Option<T>) 与 i8(T) 相加：

    //     let x: i8 = 5;
    //     let y: Option<i8> = Some(5);
    //     let sum = x + y;

    /// 很好！事实上，错误信息意味着 Rust 不知道该如何将 Option<i8> 与 i8 相加，因为它们的类型不同

    /// 总的来说，为了使用 Option<T> 值，需要编写处理每个成员的代码。你想要一些代码只当拥有 Some(T) 值时运行，允许这些代码使用其中的 T。
    /// 也希望一些代码在值为 None 时运行，这些代码并没有一个可用的 T 值。match 表达式就是这么一个处理枚举的控制流结构：它会根据枚举的成员运行不同的代码，这些代码可以使用匹配到的值中的数据。

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    fn plus_one(x: std::option::Option<i32>) -> std::option::Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
}


