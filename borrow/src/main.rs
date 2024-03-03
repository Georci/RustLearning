fn main() {
    // rust中变量的所有权是可以借用给其他函数
    // println!("Hello, world!");

    let mut names = vec!["Kenijima","Tom","Vujade"];
    // show(names);// 如果是这样的情况names的数据所有权已经被传递给v
    show2(&names);//但是如果是这样 names的数据所有权只是在show2函数运行期间将数据的所有权借给v，在函数运行完之后会将所有权还给names。但是在show运行的过程中是可以改变原来的数据的
    println!("names is {:?}",names);
    show3(&mut names);

    //rust中切片可以配合连续内存区间存储的数据结构：数组、字符串、向量
    // 首先定义一个数组
    let mut myman  = Vec::new();
    let clippers = ("PG","Harden","Leonard");//切片要求作用对象是动态类型数据，所以元组不能被切片有，因为其是静态类型数据
    myman.push("Kenijima");
    myman.push("Tom");
    myman.push("Vujade");
    let s = &myman[1..3]; // s获取切片
    //[..]获取切片中的全部元素
    println!("my friends are {:?}",s);


}
fn show(v:Vec<&str>){

}
fn show2(v:&Vec<&str>){
    // v = v + "gfor";
    println!("v is {:?}",v);
}
fn show3(v: &mut Vec<&str>){
    v[2] = "gfor";
    println!("v is {:?}",v);
}



