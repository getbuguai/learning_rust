

fn main() {
    println!("1. 输出");
    pre_install();
    shadowing();

    another_function(5, 6);

    print_fn();

    fn_if();

    fn_for();
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

// fn_if 条件
fn fn_if(){
    println!("if 条件-------------");
    let number = 3;
    if number < 5 {
        println!("条件为 true {}",number);
    } else {
        println!("条件为 false {}",number);
    }

    // else if 
    let a = 12;
    let b;
    if a > 0 {
        b = 1;
    } else if a < 0 {
        b = -1;
    } else {
        b = 0;
    }
    println!("b is {}", b);

    // 参数 if 结果为参数 (shadowing)
    let number = if b > 0 { 1 } else { -1 };
    println!("number 为 {}", number);
}

fn fn_for(){
    println!("for 循环-------------");
    let mut number = 1;
    while number != 4 {
        println!("{}", number);
        number += 1;
    }
    println!("EXIT");

    // 迭代器
    let a = [10, 20, 30, 40, 50];
    for i in a.iter() {
        println!("值为 : {}", i);
    }
    // 下标来访问数组
    for i in 0..5 {
        println!("a[{}] = {}", i, a[i]);
    }

    // loop
    let s = ['B', 'U', 'G', 'U', 'A', 'I'];
    let mut i = 0;
    loop {
        let ch = s[i];
        if ch == 'A' {
            break;
        }
        println!("\'{}\'", ch);
        i += 1;
    }

    let location = loop {
        let ch = s[i];
        if ch == 'I' {
            break i;
        }
        i += 1;
    };
    println!("\'I\' 的索引为 {}", location);
}