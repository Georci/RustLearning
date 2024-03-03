fn main() {
    // println!("Hello, world!");
    // rust中创建结构体常用的三种方法
    //1.经典C语言结构体
    /*
         struct 结构体名称{
            字段1：数据类型，
            字段2：数据类型
            。。。
         }
         let a = 结构体名称{
            变量1：数据值1，
            变量2: 数据值2
         }
         // 访问结构体数据 变量名称a.字段1
    */
    let me = KEN{
        name:String::from("Kenijima"),
        brand:String::from("Vujade"),
        buy:String::from("Vajude 004 wool jacket")
    };
    println!("me is {:?}",me);
    show(&me);
    let a = me.get_brand();
    println!("my favorite brand is {}",a);

    let s = KEN::get_anotherInstance(
        String::from("Tom"),
        String::from("Rocksteady"),
        String::from("denim jacket")
    );
    println!("s is {:?}",s);

}

fn show(s:&KEN){
    println!("{}'s brand is {},and i got his {}",s.name,s.brand,s.buy);
}

#[derive(Debug)]
struct KEN{
    name:String,
    brand:String,
    buy:String
}

//rust中的方法
impl KEN{
    fn get_brand(&self) -> &String{
        return &self.brand;
    }

    fn get_anotherInstance(name:String,brand:String,buy:String) -> KEN {
        return KEN{
            name,
            brand,
            buy,
        };
    }

}