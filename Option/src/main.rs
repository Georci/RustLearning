use std::panic;

fn main() {
    // println!("Hello, world!");
    /*
        rust中定义了所谓变体的概念，当一个数据并不确定其存在时就使用变体来解决
        1.Option作为返回值
        2.Option解包：使用match
        3.Option解包：使用unwrap
        4.Option解包：使用？
    */
    /* 函数返回一个Option的写法 */
    fn div(a:i32,b:i32) -> Option<f64>{
        if b == 0{
            None
        }else{
            Some(a as f64 / b as f64)
        }
    }

    test_match();

    test_unwrap();

    test_question_syntax()

}

/* 使用match解包Option类型变量 */
pub fn match_option(option:Option<i32>) {
    // match option {
    //     Some(val) => println!("match val = {}", val),
    //     None => println!("match Option::None")
    // }
    if let Some(val) = option{
        println!("match val = {}",val)
    }else{
        println!("match Option::None")
    }

}
fn test_match() {
    println!(">>>>> test match Option");

    match_option(None);
    match_option(Some(4));
}

/* 使用unwrap解包Option类型变量 */
fn unwrap_option(option: Option<i32>) -> i32{
    option.unwrap()
}

fn unwrap_print_option(f:fn() -> i32){
    if let Ok(val) = panic::catch_unwind(f){
        println!("val = {:?}",val);
    }
}
// Option 透过 unwrap 解构
fn test_unwrap() {
    println!(">>>>> test Option.unwrap");

    unwrap_print_option(|| unwrap_option(None));
    unwrap_print_option(|| unwrap_option(Some(6)));

    println!();
}
/* 使用?解包Option类型变量 */
// "?"相比于unwrap，在option结果是None的时候不会产生panic，而同样会有返回值
fn test_question_syntax() {
    println!(">>>>> test Option?");

    check_positive_integer(100);
    check_positive_integer(0);
    check_positive_integer(-100);

    println!();
}

fn check_positive_integer(x:i32){
    if let None = {|| {
        let var = create_option(x)?;
        println!("get positive integer :{}",var);
        Some(0)
    }}(){
        println!("{} is not positive integer", x);
    }
}

fn create_option(x:i32) -> Option<i32>{
    if x >= 0{
        Some(x)
    }else {
        None
    }
}