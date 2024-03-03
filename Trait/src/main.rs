fn main() {
    // println!("Hello, world!");

    /*
        特质trait,对标其他语言的接口，都是行为的抽象。使用trait关键字来定义。可以是具体方法，也可以是抽象的方法
        trait中包含一组方法，但是方法地第一个参数必须是&self，trait中的方法可以不实现也可以实现
        trait some_trait{
            抽象方法
            fn method1(&self)

            具体实现的普通方法
            fn method2(&self){
                方法的具体代码
            }
        }

        impl for 为每个结构体实现某个特质
    */
    let brand = Brand{
        name:String::from("Vujade"),
        maker:String::from("Kenijima"),
        price:60
    };
    // brand.show();
    test();
    let a= returns_summarizable();
    println!("{:?}",a.te());

}
struct Brand{
    name:String,
    maker:String,
    price:i32
}
trait showbrand{
    fn show(&self);
}

impl showbrand for Brand{
    fn show(&self) {
        println!("name {} maker{} price {}",self.name,self.maker,self.price);
    }
}
/*
    二、trait继承
    trait可以继承其他trait，子trait在继承父trait之后可以调用父trait中的方法，语法格式：
        trait trait_name: parent_trait_name{
        fn fn1(&self);
    }
*/
/*
    三、实现trait
    1.为某个类型实现trait，使用impl关键字
    impl trait_name for type{
        fn fn1(&self);
        //...
    }
*/

fn test(){
    let a:mystruct = mystruct{
        name: String::from("Ken"),
        brand: String::from("Vujade")
    };
    a.do_something();
    a.do_();
}
trait one{
    fn do_something(&self);
    fn do_(&self){
        println!("Kenijima");
    }
}
struct mystruct{
    name:String,
    brand:String
}

impl one for mystruct {
    fn do_something(&self) {
        println!("{}'s {}",self.name,self.brand)
    }
}

/*
    四、使用trait
    1.trait作为函数的参数
    2.trait作为返回值 trait可以作为函数的返回值类型
    fn person() -> impl Descriptive {
        //
    }
*/
trait two{
    fn test(&self);
}
fn output(a:impl one){ //任何一个实现了one 特性的实例都可以作为该函数的参数
    println!("{:?}",a.do_something());
}
fn output2(a:impl one + two){ //如果需要函数参数实现了多个特性可以使用“+”来链接特性

}

trait Summary{
    fn te(&self);
}
struct Tweet{
    username:String,
    content:String,
    reply:bool,
    retweet:bool
}

impl Summary for Tweet {
    fn te(&self) {
        println!("{},{},{},{}",self.content,self.retweet,self.reply,self.username);
    }
}
//这个函数并不会为没有实现Summary这个特性的实例赋予特性
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from( "of course, as you probably already know, people", ),
        reply: false,
        retweet: false,
    }
}


