fn main() {
    /*
        在拥有继承的语言中，如果有一个基类test，该基类实现了一个方法get_test()，那么所有继承该基类的类都可以使用或者重写该方法。但是在rust中没有继承的说法，想要做到这一点就必须使用trait对象
    */
    /*
        一、定义trait对象
        dyn 约束
        约束是trait约束或者生存期约束，可以有多个约束，多个约束之间使用“+”来链接，下面的X是一个trait
        dyn X
        dyn X+send
        dyn X + Send + Sync
        dyn X + 'static
        dyn X + Send + 'static
    */

    /*
        二、使用trait对象
        trait对象是动态尺寸类型。像所有的 DST 一样，使用trait对象，必须使用它的指针类型；例如&dyn sometrait或者Box<dynsometrait>
     */

}
