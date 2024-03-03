fn main() {

//     左闭右开的循环
    for num in 1..5{
        println!("num is {}",num);
    }

//     左闭右闭的循环
    for num2 in 1..=5{
        println!("num is {}",num2);
    }

//     数组
    // vec!关键字用于创建动态数组
    let studylist = vec![
        "《GO》",
        "《javascript》",
        "《python》"
    ];
    // 这样会打印整个数组元素
    for i in &studylist{
        println!("我现在在学的是{}",i);
    }
    println!("现在学习列表还有{:?}",studylist);



    // match匹配要说明所有可能发生的情况，.iter()的作用也是保证元素组的值不发生改变，他的意思是指借贷数组的值
    for name in studylist.iter(){
        match name{
            &"《python》" => println!("没事你还能更牛逼！"),
            &"《GO》" => println!("已经很牛逼！"),
            _ => println!("加油i")
         }
    }

    let studylist2 = vec![
        "《GO》",
        "《javascript》",
        "《python》"
    ];
    // into_iter()修饰的列表在循环中值会被move
    for name in studylist2.into_iter(){
        match name{
            "《python》" => println!("没事你还能更牛逼！"),
            "《GO》" => println!("已经很牛逼！"),
            _ => println!("加油i")
        }
    }

    let mut studylist3 = vec![
        "《GO》",
        "《javascript》",
        "《python》"
    ];

    for name in studylist3.iter_mut(){
        *name = match name{
            &mut "《python》" => {"python狗都不学，学java"},
            _ => *name //如果没有匹配则保持原来的值
        }
    }
    println!("studylist3 is {:?}",studylist3);

//     rust中的while循环就不做多的介绍了，和其他语言没有区别
//     rust中的loop循环，本身是无限循环，所以需要配合break使用

    // loop {
    //     // 执行业务逻辑
    // }
    // break; 中断的意思，就是跳出loop循环

    let mut num = 1;
    loop {
        if num > 20{
            println!("此时num的值是:{}",num);
            break;
        }
        println!("num is {}",num);
        num= num*3;
    }

}
