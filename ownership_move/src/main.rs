fn main() {
    // rust有所有权的根本原因就是rust不运行两个变量名指向同一片内存区域，这样防止资源的浪费

    let name = "Kenijima";
    let a = name;
    println!("{}",name);

    //rust 中的move包括： 1.let a = b, 2.foo(a),此时a的所有权已经术语foo函数

    // rust中的内存分为两大类：1.栈 2.堆
    // rust中的基础类型都是使用栈存放，在编译的时候大小已知
    // 堆用于存放在编译时大小未知的数据，只有在运行时才确定数据大小

    // rust中转让所有权只在堆上数据发生，基础类型数据具有copy trait
    // rust中所有权转让的情况：
    // 1.把一个变量赋值给另一个变量
    // 栈分配的整型
    let a = 88;
    // 将 `a` *复制*到 `b`——不存在资源移动
    let b = a;
    // 两个值各自都可以使用
    println!("a {}, and b {}", a, b);

    // let v1 = vec!["Go语言极简一本通","Go语言微服务架构核心22讲","从0到Go语言微服务架构师"];
    // let v2 =v1;//此时v1的数据所有权被转移到v2，所以如果下面再次使用v1会报错
    // println!("{:?}",v1);
    // 2.把变量当作参数传递
    let studyList = vec!["Go语言极简一本通","Go语言微服务架构核心22讲","从0到Go语言微服务架构师"];
    //studyList 拥有堆上数据管理权
    let studyList2 = studyList;
    // studyList 将所有权转义给了 studyList2
    show(studyList2);
    // studyList2 将所有权转让给参数 v,studyList2 不再可用。
    // println!("studyList2 {:?}",studyList2);
    //studyList2 已经不可用。

    //3.函数中的返回值
    let studyList3 = vec!["Go语言极简一本通","Go语言微服务架构核心22讲","从0到Go语言微服务架构师"];
    let studyList4 = studyList3;
    let result = show2(studyList4);// studyList4的数据所有权被转移到函数参数之后返回给result
    println!("result {:?}",result);
    //输出 result ["Go语言极简一本通", "Go语言微服务架构核心22讲", "从0到Go语言微服务架构师"]
}
fn show(v:Vec<&str>){

}
fn show2(v:Vec<&str>) -> Vec<&str>{
    println!("v is {:?}",v);
    return v;
}
