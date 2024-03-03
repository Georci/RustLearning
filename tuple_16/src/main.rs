fn main() {
    // 元组：元组和数组最大的区别在于 1.元组是复合类型，同一个元组可以支持存放多种类型数据 2.元组只有定长，没有动态元组

    let mut test = (12,"kenijima");
    let mut shuzu = vec![1,2,3,4];
    let mut a = 1;
    // let testLength = test.len();
    let shuzuLength = shuzu.len();
    println!("test is {:?}",test);
    println!("shuzu is {:?}",shuzu);

    // println!("test's Length is {:?}",testLength);
    println!("shuzu's Length is {:?}",shuzuLength);

    // 访问元组元素 元组变量.索引数字,索引从0开始
    println!("{}",test.1);

    // 参数传递的方式调用其他函数
    showTuple2(&mut test);
    showTuple(&mut shuzu);
    // showTuple(&mut a);
}


// 被调用函数在执行参数调用时，要指明参数的类型
fn showTuple(shuzu:&mut Vec<i32>){

}

fn showTuple2(test:&mut (i32,&str)){
    println!("i need {} to {}",test.0,test.1);

    // 元组解构
    let (num, people) = test;
    println!("i need {} to {}",num,people);
}
