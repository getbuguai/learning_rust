

fn main() {
    println!("1. 输出");
    pre_install();
    shadowing();

    another_function(5, 6);

    print_fn();
}

// pre_install 输出
fn pre_install(){
    println!("Hello Get Buguai");
    let a ="string";
    println!("a is {0}, a again is {0}", a);
    println!("{{}}");
    print!("格式化输出： 213131321\n");
}

// shadowing
// 重影的概念与其他面向对象语言里的"重写"（Override）或"重载"（Overload）是不一样的。
// 重影就是刚才讲述的所谓"重新绑定"，之所以加引号就是为了在没有介绍这个概念的时候代替一下概念。
fn shadowing(){
    print!("重影==================");
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
    let  s = "123";
    println!("The value of x is: {}", s.len());
}

fn another_function(x: i32, y: i32) {
    println!("带参数的函数-----------------");
    println!("x 的值为 : {}", x);
    println!("y 的值为 : {}", y);
}

// 函数体的语句和表达式，作用域
fn print_fn(){
    println!("函数赋值-------------");
    let x = 5;

    let y = {
        let x = 3;
        // y 的值为 新的 x 的值
        x + 1
    };

    println!("x 的值为 : {}", x);
    println!("y 的值为 : {}", y);

    // 函数参数
    fn five() -> i32 {
        5
    }
    println!("five() 的值为: {}", five());

    fn add(a: i32, b: i32) -> i32 {
        return a + b;
    }
    println!("add() 的值为: {}", add(12,78));
}