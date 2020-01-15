///标准库的比较，Ordering枚举包含Less， Greater和Equal
use std::cmp::Ordering;
///导入标准库的输入输出库
use std::io;
use std::ops::{Range, RangeBounds};

///引入rand库的Rng特质
use rand::Rng;

/// 引用和借用：https://dreamylost.cn/rust/Rust-Rust%E5%AD%A6%E4%B9%A0%E4%B9%8B%E5%BC%95%E7%94%A8%E4%B8%8E%E5%80%9F%E7%94%A8.html
/// 所有权：https://dreamylost.cn/rust/Rust-%E6%89%80%E6%9C%89%E6%9D%83.html
fn main() {
    println!("Hello, world!");
    variables();
    println!("====================");
    data_type();
    println!("====================");
    another_function(12);
    println!("====================");
    expr_function();
    println!("====================");
    println!("{}", five());//打印字符串，不能直接println!(five())
    println!("====================");
    branchs();
    println!("====================");
    println!("斐波那契第20项是：{}", fib(20));
    println!("====================");
    println!("斐波那契第20项是：{}", fib_2(20));
    println!("====================");
    string_from();
    println!("====================");
    return_function();
    println!("====================");
    tuple_function();
    println!("====================");
    copy_function();
    println!("====================");
    point_function();
    println!("====================");
    try_change();
    println!("====================");
    empty_point();
    guessing_game();
}
//下面为了编译，将错误或多余代码注释了。

fn empty_point() {

//    let reference_to_nothing = dangle();
    let reference_to_nothing = no_dangle();
    fn no_dangle() -> String {
        String::from("hello")// 直接反回函数的值，不能加分号
    }
    //编译报错，因为s是在dangle内部创建的，所以当dangle的代码完成时，将释放s。但是我们试图返回对它的引用。这意味着该引用将指向无效的String。Rust不允许我们这样做。
//    fn dangle() -> &String {
//        let s = String::from("hello");
//        &s
//    }
}

fn try_change() {
    //必须都是mut的，否则编译就会报错，不可变，无法被改变
    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }
    let mut s = String::from("hello");
    change(&mut s);


    let mut s = String::from("hello");
    let r1 = &mut s;
//    let r2 = &mut s;//可变引用只能被出借一次，这里将会编译报错
//    println!("{}, {}", r1, r2);


    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // r1在这里超出范围，因此我们可以毫无问题地进行新引用。
    let r2 = &mut s;//正常使用，虽然上面已经用过s


    let mut s = String::from("hello");
    let r1 = &s; // 没问题，与上面两次mut出借不一样，这里是没有mut，所以对于不可变引用，可以使用多次次，且不可在拥有不可变引用时同时拥有可变引用
    let r2 = &s; // 没问题
//    let r3 = &mut s; // 有问题，不可变在后面却是可变，不允许，编译报错
//    println!("{}, {}, and {}", r1, r2, r3);


    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
// 在此之后不再使用r1和r2

    let r3 = &mut s; // 没问题，因为r1 r2进入println! 并且在此之后会失效，与所有权有关。
    println!("{}", r3);
}

fn point_function() {
    fn calculate_length(s: &String) -> usize {
        s.len()
    }
    let s1 = String::from("hello");
    //类似c/c++传递指针/引用
    let len = calculate_length(&s1);
    //s1在之后还能使用
    println!("The length of '{}' is {}.", s1, len);
}

fn copy_function() {
    let x = 5;
    //基本类型在移动时使用copy，x不会失效。
    let y = x;
    println!("x = {}, y = {}", x, y);

    //使用clone克隆数据，目前先理解为深拷贝
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn tuple_function() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len(); // len() 返回字符串的长度
        (s, length)
    }
}

fn return_function() {
    let s1 = gives_ownership();         // lets_ownership移动其返回值到s1中

    let s2 = String::from("hello");     // s2进入范围

    let s3 = takes_and_gives_back(s2);  // s2被移入takes_and_gives_back,  takes_and_gives_back的返回值被移动到s3
    println!("{},{}", s1, s3);

    fn gives_ownership() -> String {             // gives_ownership会其返回值移动到调用它的函数中
        let some_string = String::from("hello"); // some_string进入范围
        some_string                              // 返回some_string字符串并移到调用函数
    }

    // take_and_gives_back将获取一个String并返回一个
    fn takes_and_gives_back(a_string: String) -> String { // a_string进入范围
        a_string  // 返回a_string并移至调用函数
    }
}

fn string_from() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() 将文字附加到字符串

    println!("{}", s); //打印 hello, world!


    let s = String::from("hello");  // s进入范围

    takes_ownership(s);          // s的值移动到函数，所以在这里不再有效
//    println!("{}", s);//编译错误：value borrowed here after move。出借后的s被移动，后续不可用

    let x = 5;                      // x进入范围
    makes_copy(x);            // x将移动到函数
    // 但是i32是Copy，所以之后还可以使用
    println!("{}", x);//正常打印

    fn takes_ownership(some_string: String) {
        println!("{}", some_string);
    } //在这里，some_string超出范围并调用`drop`。内存释放

    fn makes_copy(some_integer: i32) {
        println!("{}", some_integer);
    }
}

fn fib(n: i32) -> i32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn fib_2(n: i32) -> i32 {
    let mut a = 0;
    let mut b = 1;
    let mut c = 0;
    if n == 0 || n == 1 {
        n
    } else {
        for number in 2..(n + 1) {
            c = a + b;
            a = b;
            b = c;
        }
        c
    }
}

//控制流
fn branchs() {
    let number = 3;
    //表达式结果必须是bool类型，不像c会自动将非bool转化为bool
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    //处理多个if
    let number = 6;//阴影，遮盖了前面的number

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    //与Scala一样，可以将if表达式的结果赋值给变量（这里的变量一般是指不可变的变量，虽然绕口，但是确实是事实）
    let condition = true;
    //从每个分支取得的if的返回值必须是同一类型，否则编译报错
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);

    //循环
    loop {
        println!("again!");
        break;//这个分号可省
    }
    //从循环中返回值
    let mut counter = 0;
    //循环赋值给变量
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        };//这个分号可省
    };

    //分号的使用还不清晰明确，后面再看
    //暂时理解为，赋值给变量的代码块需要使用分号短句，不赋值可以不用分号，而表达式本身就是直接返回，使用分号反而不行。（return显示指定返回值）
    println!("The result is {}", result);

    //while循环
    let mut number = 3;
    //使用while可以减少大量的if else break
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    };//这个分号可以省略
    println!("LIFTOFF!!!");

    //while变量数组
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    //使用for循环更加简单
    //rust常用for，因为rust不会有迭代器失效的问题
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    //使用倒数
    for number in (1..4).rev() {
        //输出3!2!1!LIFTOFF!!!  print是没有换行的，与其他语言一致
        print!("{}!", number);
    }
    println!("LIFTOFF!!!");
}


//具有返回值的rust函数
fn five() -> i32 {
    //这里同样，由于需要返回值为i32类型，增加了分号表示语句，没有返回值（实际是空括号），所以导致类型不一致，编译会报错
    5
}

fn expr_function() {

    //赋值需要返回值，rust语句没有返回值，不同于其他语言赋值可以连用
    // let x = (let y = 6);

    let x = 5;

    let y = {
        let x = 3;
        x + 1 //返回x+1，且不能用分号，有分号表示这个是语句，没有返回值，无法赋值给y
    };

    println!("The value of y is: {}", y);
}

fn another_function(x: i32) {
    //传参数的rust函数，与Scala一样，名称: 类型
    println!("The value of x is: {}", x);
}

//rust不关注函数与main的顺序问题
fn data_type() {

    //--release模式下，整数溢出将会变为最小值
    //在u8(0-255)类型下，256变为0，257变为1，依此类推


    //默认浮点类型是f64，相当于Java double，IEEE754标准
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    //数值运算，与其他语言相同，类型可以自动推断，不用指定类型
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    let t = true;
    //显示指定类型
    let f: bool = false;

    //字符类型，Unicode，4bytes
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    //元组类型，与Scala基本相同，可以推断出类型
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    //提取出元组的每个值
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    //使用 .获取元组的值，从0下标开始
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    //数组类型，一般在只有固定元素个数的时候使用
    let array = [1, 2, 3, 4, 5];

    //初始化数组的第二种方法
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    //等价于let a = [3, 3, 3, 3, 3];，意为5个3构成的数组
    let a = [3; 5];

    //访问数组，同样是从0下标开始
    let first = a[0];
    let second = a[1];

    //Rust通过立即退出而不是允许内存访问并继续操作来保护您免受此类错误的侵害
    let element = a[0];//若下标大于数组索引则运行时检查并报错退出"error: index out of bounds: the len is 5 but the index is 10"
}

fn variables() {
    //默认i32，带符号32位整数
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6; 不可变的，编译不过
    // println!("The value of x is: {}", x);

    let mut y = 6;
    println!("The value of y is: {}", y);
    y = 7;//可变的变量
    println!("The value of y is: {}", y);
    //常量，必须指定类型，不可省略
    const MAX_POINTS: u32 = 100_000;
    println!("The value of const value is: {}", MAX_POINTS);

    // 阴影允许定义变量与前面重名，前者被遮蔽
    //mut与shadowing区别：后者将创建一个新的变量，因此可以改变类型，使用相同的名称，常见用法如下：
    //let spaces = "   ";
    // let spaces = spaces.len();//使用相同名称但类型已经发生变化
    // 但是对于mut则不能，spaces虽然是可变的，但是类型是字符串类型的
    // let mut spaces = "   ";
    // spaces = spaces.len();
    let i = 5;
    let i = x + 1;
    let i = x * 2;

    println!("The value of x is: {}", i);
}

fn guessing_game() {
    println!("Guess the number!");

    //thread_rng一个在当前执行线程本地且由操作系统播种的随机数生成器
    let secret_number = rand::thread_rng().gen_range(1, 101);

    //    println!("The secret number is: {}", secret_number);
    loop {
        println!("Please input your guess.");

        //变量默认是不可变的。使用mut表示变量是可变的，定义成let foo = 5; 则是不可变。
        let mut guess = String::new();//关联函数，在类型上实现。一些语言称为静态方法。该函数创建了一个空串

        //没有使用use，则这里需要写成 std::io::stdin
        //&表示该参数是一个引用，Rust的主要优势之一是使用引用的安全性和便捷性
        //&使您代码的多个部分可以访问同一条数据，而无需将该数据多次复制到内存中
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        //无法比较数值与字符串需要转化为数值，Rust默认为i32
        //Rust允许我们用新的值遮盖以前的值guess。此功能通常用于要将值从一种类型转换为另一种类型的情况。
        //阴影使我们可以重用guess变量名，而不是强迫我们创建两个唯一变量，例如guess_str和guess。
        //前面的guess是可变的，这个是不可变的。
        //    let guess: u32 = guess.trim().parse().expect("Please type a number!");//类型不明确，必须指定具体类型
        //println!是宏
        println!("You guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            //遇到无效输入直接跳过
            Err(_) => continue,
        };

        println!("Please input your guess.");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                //猜到正确数字后退出循环
                break;
            }
        }
    }
}
