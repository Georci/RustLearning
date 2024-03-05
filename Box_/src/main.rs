use std::fmt::Formatter;

fn main() {

    /*
        如果一个结构体，实现了deref和drop的Trait，那他们就不是普通结构体了Rust提供了堆上存储数据的能力并把这个能力封装到了Box中。
        把栈上的数据搬到堆上的能力，就叫做装箱。Box可以把数据存储到堆上，而不是栈上，box装箱，栈还是包含指向堆上数据的指针。
        修改box数据有两种方式，第一种是修改box指向的数据，第二种是修改box指向的位置
    */
    // println!("Hello, world!");
    let price1 = 158;           // 值类型数据
    let mut price2 = Box::new(price1); // price2 是一个智能指针，指向堆上存储的数据 158
    println!("{}",158==price1);
    // 此时price2的类型是box<integer>，不能直接与integer类型的数据相比较
    println!("{}",158==*price2); // 为了访问 price2 存储的具体数据，需要解引用
    price2 = Box::from(4);// 直接修改box的值
    //修改box指向的位置,也就是说修改box指针，让其指向一个新的对象
    *price2 = 4;
    // println!("{}",price2);

    let a = 6;
    let b = Box::new(a);
    println!("{}",b);

    let t = test{
        name:String::from("Kenijima"),
        brand:String::from("Vujade")
    };
    t.printinfo()


}
/*
        下面介绍impl关键字，impl关键字为类型实现 方法 和 特性
    */

// 为结构体实现方法
struct test{
    name:String,
    brand:String
}

impl test{
    fn printinfo(&self){
        println!("{}'s brand is {}",self.name,self.brand);
    }
}
// 为结构体实现一个trait
impl std::fmt::Display for test {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

