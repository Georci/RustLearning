// rust中的条件语句用法与一般语言的条件语句差不多，重点介绍一下不一样的地方，match语法

// match的使用方式有点像Switch
// match variable_expression {
// constant_expr1 => {
// // 语句;
// },
// constant_expr2 => {
// // 语句;
// },
// _ => {
// // 默认
// // 其它语句
// }
// };
fn main(){
    let code = "10087";

    let choose = match code{
        "10086" => {"移动"},
        "10010" => {"联通"},
        _ => {"Unknow"}
    };

    println!("选择{}",choose);
}

