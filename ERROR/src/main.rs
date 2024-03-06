use std::fs;

// use crate::std::result::Result;
fn main() {
    // println!("Hello, world!");

    /* Rust中的错误处理，使用Result枚举来实现的 */
    // 如果无错误则使用Ok（T）来返回，否则使用Err（E）来返回。
    // “？”是Rust中的语法糖，如果函数被正常调用则返回其值并使用unwrap对其进行解包，否则返回Err(E)，对错误不进行解包

    // run();

    read_file();
    // Rust是强类型语言，如果在一个函数中使用了多个？返回了错误，并且他们的类型是不同的还需要对返回的错误类型进行转换，转为相同的类型！
}
// pub enum Result<i64,MyError>{
//     Ok(i64),
//     Err(MyError),
// }

#[derive(Debug)]
pub enum MyError {
    ReadError(String),
    ParseError(String),
}
//
// pub fn add(num: i64) -> Result<i64, MyError> {
//     if num < 0 {
//         Err(MyError::InvalidId(String::from("Invalid num!")))
//     } else {
//         Ok(num + 100000)
//     }
// }
//
// fn run() -> Result<(),MyError>{
//     let res = add(-1)?;
//     println!("{:?}",res);
//     Ok(())
// }


// fn read_file1() -> Result<i64, MyError> {
//     // Error: Could not get compiled!
//     let content = fs::read_to_string("/tmp/id")?;
//     let id = content.parse::<i64>()?;
// }


fn read_file() -> Result<i64, MyError> {
    // Error: Could not get compiled!
    // let content = fs::read_to_string("/tmp/id")?;
    // let id = content.parse::<i64>()?;

    // Method 1: Handling error explicitly!
    let content = match std::fs::read_to_string("/tmp/id") {
        Ok(content) => content,
        Err(err) => {
            return Err(MyError::ReadError(format!("read /tmp/id failed: {}", err)));
        }
    };
    let content = content.trim();
    println!("read content: {}", content);

    // Method 2: Use map_err to transform error type
    let id = content
        .parse::<i64>()
        .map_err(|err| MyError::ParseError(format!("parse error: {}", err)))?;

    Ok(id)
}