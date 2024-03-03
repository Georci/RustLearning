fn main() {
    // println!("Hello, world!");
    // 迭代器就是把集合中的所有元素按照顺序一个接一个的传递给逻辑处理。迭代器有两个特质：1.iter() 2.next()

    let v = vec!["Kenijima","Tom","Georci"];
    let mut it = v.iter();
    println!("{:?}",it);
    println!("{:?}",it.next());
    println!("{:?}",it.next());
    println!("{:?}",it.next());
    println!("{:?}",it.next());

    let iter = v.iter();
    println!("{:?}",iter);

    //闭包，闭包就是在函数内创建立即调用另外一个函数，闭包是一个匿名函数，又称之为内联函数
    /*
        普通函数
        fn 函数名称 -> 返回值{
            //业务逻辑
        }

        闭包
        |参数列表|{
            //业务逻辑
        }

        无参数闭包
        ||{
            //业务逻辑
        }
    */
    // let double = |x|{x*2};
    //
    // let add = |a,b|{a+b};

    let print_hello = || println!("Hello, world!");

    apply(print_hello);

    let closure = create_closure();
    closure();
}
fn apply<F>(f:F)
where
    F:FnOnce(),
{
    f();
}

fn create_closure() -> impl Fn() {
    let x = 5;
    move || println!("The value of x is: {}", x)
}
