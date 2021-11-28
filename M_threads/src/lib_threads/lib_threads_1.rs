
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

fn check4() {
    println!("check4");
    let v = vec![1,2,3];

    //  v ownership moved inside thread closure
    let handle = thread::spawn(move || {
        println!("\t{:?}",v); 
    });

    // println!("{:?}", v); not allowed

    handle.join().unwrap();
}

use std::sync::mpsc;

fn passing1() {
    println!("passing1");
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msg = String::from("hi");
        tx.send(msg).unwrap();
    });

    let recvd = rx.recv().unwrap();
    println!("\t{}", recvd);
}

fn passing2() {
    println!("passing2");
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msgs = vec![
            String::from("hello"),
            String::from("from"),
            String::from("the"),
            String::from("other"),
            String::from("side"),
        ];
        tx.send(msgs).unwrap();
    });

    let recvd = rx.recv().unwrap();
    println!("\t{:?}", recvd);
    for msg in recvd {
        println!("\t{}", msg);
    }
}

fn passing3() {
    println!("passing3");
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msgs = vec![
            String::from("hello"),
            String::from("from"),
            String::from("the"),
            String::from("other"),
            String::from("side"),
        ];
        for msg in msgs {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    while let Ok(recvd) = rx.recv() {
        println!("\t{:?}", recvd);    
    };
    // for msg in recvd {
    //     println!("\t{}", msg);
    // }
}

fn passing4() {
    println!("passing4");
    let (tx, rx) = mpsc::channel();

    let tx_copy = tx.clone();

    thread::spawn(move || {
        let msgs = vec![
            String::from("hello"),
            String::from("from"),
            String::from("the"),
            String::from("other"),
            String::from("side"),
        ];
        for msg in msgs {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let msgs = vec![
            String::from("I"),
            String::from("must"),
            String::from("have"),
            String::from("called"),
            String::from("a"),
            String::from("thousand"),
            String::from("times"),
        ];
        for msg in msgs {
            tx_copy.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    loop {
        match rx.recv() {
            Ok(recvd) => { 
                println!("\tMsg={:?}", recvd);
            }
            Err(err) => {
                println!("\tErr={:?}", err);
                break;
            }
        }
    }
}

pub fn lib_threads_1_runner() {
    println!("lib_thread_1_runner");
    check1();
    check2();
    check3();
    check4();
    passing1();
    passing2();
    passing3();
    passing4();
}
