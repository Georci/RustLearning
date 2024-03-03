fn main() {
    // println!("Hello, world!");
    /* enum Option<T>{
        Some<T>, //用于返回一个值
        None    // 用于返回null，用None来代替
        }
    */
    let wuxizhi =KEN::Kenijima;
    println!("{:?}",wuxizhi);
    matchname(KEN::Kenijima);
    matchname(wuxizhi)
}

#[derive(Debug)]
enum KEN{
    Kenijima,
    Tom
}

fn matchname(r:KEN){
    match r{
        KEN::Tom => {
            println!("Tom is good");
        },
        KEN::Kenijima => {
            println!("Ken is good");
        }
    }
}