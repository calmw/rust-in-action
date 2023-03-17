use std::thread;   // 把多线程能力导入当前的局部作用域

fn main() {
    let mut data = 100;

    thread::spawn(|| { data = 500; });    // thread::spawn 接收一个闭包作为参数
    thread::spawn(|| { data = 1000; });   // 线程的调度是由操作系统而不是程序决定的，因此根本无法知道先定义的那个线程会不会率先执行

    println!("{}", data);
}