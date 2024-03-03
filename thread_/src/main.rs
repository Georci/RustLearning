use std::thread;
use std::time::Duration;

fn main() {
    /*
        现代的操作系统，是一个多任务操作系统，系统可以管理多个程序的运行，一个程序往往有一个或多个进程，而一个进程则有一个或多个线程.
        让一个进程可以运行多个线程的机制叫做多线程。
        一个进程一定有一个主线程，主线程之外创建出来的线程叫 子线程。主线程结束时，子线程也就自动结束了
        多线程（并发）编程的一个重要思想就是 程序不同的部分可以同时独立运行互不干扰。
    */
    // thread::spawn(|| {
    //     for i in 1..10{
    //         println!("子线程{}",i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });
    //
    // for i in 1..5{
    //     println!("主线程{}",i);
    //     thread::sleep(Duration::from_millis(1));
    // }
    // thread::sleep()会让线程睡眠一段时间，某个线程睡眠的时候，会让出cpu，可以让不同的线程交替执行。

    // 可以使用Join方法让主线程等待子线程执行完毕
    let handler = thread::spawn(|| {
        for i in 1..10{
            println!("子线程{}",i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5{
        println!("主线程{}",i);
        thread::sleep(Duration::from_millis(1));
    }
    handler.join().unwrap();//此时，主线程执行完毕会等待子线程
}
