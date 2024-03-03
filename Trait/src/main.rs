fn main() {
    // println!("Hello, world!");

    /*
        特质trait,对标其他语言的接口，都是行为的抽象。使用trait关键字来定义。可以是具体方法，也可以是抽象的方法
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
    brand.show()

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