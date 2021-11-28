
use std::{thread, time::Duration};

fn check1() {
    println!("check1");
    thread::spawn(|| {
        for i in 1..10 {
            println!("\tnum_1=`{}`", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
            println!("\tnum_2=`{}`", i);
            thread::sleep(Duration::from_millis(1));
    }
}

fn check2() {
    println!("check2");
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("\tnum_1=`{}`", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
            println!("\tnum_2=`{}`", i);
            thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}

fn check3() {
    println!("check3");
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("\tnum_1=`{}`", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();
    for i in 1..5 {
            println!("\tnum_2=`{}`", i);
            thread::sleep(Duration::from_millis(1));
    }
}

pub fn lib_threads_1_runner() {
    println!("lib_thread_1_runner");
    check1();
    check2();
    check3();
}
