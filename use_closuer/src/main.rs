fn main() {
    // 闭包的使用
    // 定义一个最简单的闭包
    let add = |x,y| {x+y};
    let result = add(3,2);
    println!("{}",result);

    //将闭包变量作为参数
    receives_closure(add);
    //也可以将闭包直接作为参数
    let y = 2;
    receives_closure2(|x|{x+y});

    let result = do1(add,5);
    println!("result(1) => {}",result(1));

    let result2 = do2(add,5);
    println!("result(2) => {}",result2(8));
}


// 定义receives_closure函数，该函数的参数closure，类型是泛型，
fn receives_closure<F>(closure:F)
    where
        F:Fn(i32,i32) -> i32 // 对该泛型提出要求，要求其必须实现一个闭包，且该闭包参数是两个i32类型，返回值是i32类型
{
    let result = closure(3,5);
    println!("闭包作为参数执行结果=>{}",result);
}

fn receives_closure2<F>(closure:F)
    where
        F:Fn(i32) -> i32{
    let result = closure(1);
    println!("{}",result);
}

// 这个函数是拥有泛型参数的函数，泛型参数为f，还有一个i32类型的参数，且该函数的返回值是一个参数为i32类型的闭包，且该闭包返回值为i32类型
fn do1<F>(f:F,x:i32) -> impl Fn(i32) -> i32
    where
        F:Fn(i32,i32) -> i32{ // 对泛型F做出一些限制，要求其是一个闭包，且闭包返回一个i32类型的数据
    move |y| f(x,y) //返回一个闭包，且该闭包有一个参数，再调用f
}
fn do2<F,X,Y,Z>(f:F,x:X) -> impl Fn(Y) -> Z
    where
        F:Fn(X,Y) -> Z,
        X:Copy{
    move |y| f(x,y)
}