fn main() {
    // println!("Hello, world!");

    /*
    mod模块样式如下，mod模块中默认所有函数是private可见性，如果想要mod外部的函数访问mod内部的函数，可以使用pub关键字来声明mod pub mod module_name
    而在公开模块中，模块中的函数可以使用pub关键字来将其声明为公开函数，当然也可以不使用pub关键字，此时该函数仍然是一个私有函数
    使用模块：use 公开模块::函数名称
        mod moudle_name{
            fn function_name(){

            }
        }

        同样模块之间也允许嵌套,调用的时候也会比较负载 use mod1::mod2::mod3::function_name
        pub mod te1{
            pub mod te2{
                pub mod te3{
                    fn function_name(){}
                }
            }
        }
    */
}

