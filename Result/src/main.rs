fn main() {
    // println!("Hello, world!")

    /*
        Result 也有两种变体：OK<T>(value) , Err<E>(why),
    */
}
/* Result解包，使用match */
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NonPositiveLogarithm,
    NegativeSquareRoot,
}
//定义别名作为更具体的Result
type MathResult = Result<f64,MathError>;

fn div(x:f64,y:f64) -> MathResult{
    if y == 0.0{
        Err(MathError::DivisionByZero)
    } else {
        Ok(x / y)
    }
}
// 平方根
fn sqrt(x: f64) -> MathResult {
    if x < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}

// 取对数
fn ln(x: f64) -> MathResult {
    if x <= 0.0 {
        Err(MathError::NonPositiveLogarithm)
    } else {
        Ok(x.ln())
    }
}
// 最后使用多层的match进行解析
// sqrt(ln(x / y)) 测试
fn test_ops(x: f64, y: f64) -> MathResult {
    match div(x, y) {
        Err(e) => Err(e),
        Ok(ratio) => {
            match ln(ratio) {
                Err(e) => Err(e),
                Ok(ln) => {
                    match sqrt(ln) {
                        Err(e) => Err(e),
                        Ok(sqrt) => Ok(sqrt)
                    }
                }
            }
        }
    }
}
// 上面这个函数是使用match进行解包的写法，还可以使用unwrap和”？“方式进行解包
// 上面整个函数使用？来解包的话如下所示：
fn test_ops_simple(x: f64, y: f64) -> MathResult {
    let ratio = div(x, y)?;
    let ln = ln(ratio)?;
    let sqrt = sqrt(ln)?;

    Ok(sqrt)
}
// 甚至可以写成
fn test_ops_simple2(x: f64, y: f64) -> MathResult {
    Ok(sqrt(ln(div(x, y)?)?)?)
}

fn test_ops_result(x: f64, y: f64) {
    match test_ops(x, y) {
        Err(e ) => {
            println!("test({}, {}) fail: {:?}", x, y, e);
        }
        Ok(val) => {
            println!("sqrt(ln({} / {})) = {}", x, y, val);
        }
    }
}


