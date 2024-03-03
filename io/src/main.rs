use std::env::args;
use std::io::Write;

fn main() {
    // println!("Hello, world!");
    //I/0 就是输入(input)/输出(Output)。Rust 语言 10 输入输出的三大块内容:读取数据、写入数据、命令行参数,
    // rust标准库IO输入输出以下两个trait 1.read 2.write
    /*
        std::io::stdin()返回标准输入流stdin的句柄。read_line()是标准入流stdin的句柄上的一个方法，从标准输入流中读取一行数据。返回值是一个Result枚举，
        而unwrap()则是一个帮助方法，用于简化可恢复
    */
    let mut in_word = String::new();
    let result = std::io::stdin().read_line(&mut in_word).unwrap();
    println!("你输入的数据是{}",in_word);
    println!("写入的字节数是{}",result);

    /*

    */
    let result1 = std::io::stdout().write("Kenijima".as_bytes()).unwrap();
    println!("输入的字节数为：{}",result1);

    let input_args = std::env::args();
    for arg in input_args{
        println!("命令行参数：{}",arg);
    }

}
