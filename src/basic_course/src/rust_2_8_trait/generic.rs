use common::outer_print;
use crate::add;

#[outer_print("泛型")]
pub fn intro(){
    // fn add<T>(a:T,b:T)->T{
    //     a+b
    // }
    // println!("a+b={:}",add(1,2))
    // 将之前的代码改成上面这样，就是 Rust 泛型的初印象，这段代码虽然很简洁，但是并不能编译通过，我们会在后面进行详细讲解，现在只要对泛型有个大概的印象即可。
}

#[outer_print("泛型详解")]
pub fn generic_explained(){
    /// 上面代码的 T 就是泛型参数，实际上在 Rust 中，泛型参数的名称你可以任意起，但是出于惯例，我们都用 T ( T 是 type 的首字母)来作为首选，这个名称越短越好，除非需要表达含义，否则一个字母是最完美的。
    // 使用泛型参数，有一个先决条件，必需在使用前对其进行声明：
    fn largest<T>(list: &[T]) -> T {
        
    }

}

#[outer_print("结构体中使用泛型")]
pub fn generic_struct(){

}

#[outer_print("枚举中使用泛型")]
pub fn generic_enum(){

}
#[outer_print("方法中使用泛型")]
pub fn generic_method(){

}

#[outer_print("const泛型")]
pub fn generic_const(){

}

#[outer_print("泛型的性能")]
pub fn generic_performance(){

}