use std::collections::{HashMap, HashSet};

fn main() {
    // println!("Hello, world!");
    // 集合的创建方式有两种，第一种是通过new vec()的方式 第二种是通过宏的方式
    let mut collections = Vec::new();
    collections.push("Kenijima");
    collections.push("Tom");
    collections.push("Goerci");
    println!("{:?}",collections);

    let mut json = vec!["kenijima","Tom","Goerci"];

    //通过remove方法移除集合中指定位置的元素，并将该移除值返回
    let x = json.remove(2);
    println!("{:?}",json);
    println!("{}",x);

    //在向量中查找指定元素
    if json.contains(&"kenijima"){
        println!("找到了");
    }

    //遍历向量
    for item in json{
        println!("wo we are{}",item);
    }
    /*
        哈希表
    */
    let mut process = HashMap::new();

    //使用insert向哈希表中插入元素
    process.insert("kenijima",1);
    process.insert("tom",2);
    process.insert("georci",3);
    println!("{:?}",process);

    //通过get方法从哈希表中获取指定键对应的值
    match process.get("Tom") {
        Some(v) => {
            println!("找到了");
        }
        None => {
            println!("找不到");
        }
    }

    //哈希表是无序的，下面介绍查找哈希表中是否存在指定的值
    if process.contains_key("tom"){
        println!("hooo!");
    }
    for item in process.iter(){
        println!("key {} value {}",item.0,item.1);
    }
    /*
        HashSet 相同数据类型的集合，没有重复的值。如果存在相同的值，插入会失败。
        let mut 集合名称 = hashset::new();
    */
    let mut hashset = HashSet::new();
    hashset.insert("Json");
    hashset.insert("brown");
    hashset.insert("A");
    println!("{:?}",hashset);
}

