use std::fmt::{Display, Formatter};

fn main() {
    // println!("Hello, world!");
    //泛型是运行时指定数据类型的一种机制。好处是通过高度的抽象，使用一套代码应用多种数据类型。
    //rust中的泛型主要包含泛型集合、泛型结构体、泛型函数、范型枚举 和 特质
    // rust语法使用<T>来实现泛型，其中T可以是任意数据类型
    // 泛型集合
    // let mut v:Vec<i32> = vec![1,2,3,"kenijima"];
    // let mut v1 = vec![1,2,3,"kenijima"];
    // println!("{:?}",v);//会报错
    // println!("{:?}",v1) //也会报错


    //泛型结构体,在定义结构体时，可以保留每个字段的类型
    /*
        struct 结构体名称<T>{
            字段:T
        }
    */
    let mut a:Data<i32> = Data{value:100};
    let mut b:Data<f32> = Data{value:100.1212};
    // println!("{:?}",a);
    // println!("{:?}",b);
    let brand:Brand = Brand{
        name:String::from("Kenijima"),
        maker:"Vujade".to_string(),
        price:60
    };
    //在这个地方我们使用一个实现了Display特质的结构体传入函数show2
    show2(brand);
}

/*
    1.泛型函数 语法格式
    fn function_name<T[:trait_name]>(param1:T,[other_params]){
        函数实现代码
    }
    fn max<T>(array: &[T]) -> T //这行代码的意思是<T>说明函数是一个泛型函数，函数有一个参数，参数类型是一个包含多个泛型数据的切片，且该函数返回值是一个泛型数据
    2.泛型结构体
    struct struct_name<T> {
        field:T
    }
    3.泛型枚举
    enum Option<T> {
        Some(T),
        None
    }
    enum Result<T, E> {
        Ok(T),
        Err(E)
    }
    4.在trait中使用泛型

*/
//泛型函数,下面这个函数表示传入show2的函数接受一个泛型类型的参数，且该泛型一定实现Display的特质，否则会报错
fn show2<T:Display>(t:T){
    println!("{}",t);
}
struct Brand{
    name:String,
    maker:String,
    price:i32
}

impl Display for Brand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        println!("name {} maker{} price {}",self.name,self.maker,self.price);
        let r = Result::Ok(());
        return r;
    }
}


//#[derive(Debug)]是属性宏，他告诉编译器自动为结构体或枚举实现Debug trait，允许程序使用{：？}将其打印
#[derive(Debug)]
struct Data<T>{
    value:T
}
