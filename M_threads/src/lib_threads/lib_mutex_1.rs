// Mutex

//#![feature(mutex_unlock)]

use std::sync::Mutex;
use std::sync::MutexGuard;

fn check1() {
	println!("check1");
	let m: Mutex<u32> = Mutex::new(5);

	{
		let mut num: MutexGuard<u32> = m.lock().unwrap();
		*num += 50;
		println!("\t{}", num);
		//Mutex::unlock(num);
	}

	println!("\t{:?}", m);

	if let Ok(val) = m.into_inner() {
		println!("\t{:?}", val);
	}
}

use std::thread;
//use std::rc::Rc;
use std::sync::Arc;

fn check2() {
	println!("check2");
	let counter:Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
	let mut handles: Vec<thread::JoinHandle<()>> = vec![];

	for _ in 0..20 {
		let counter = Arc::clone(&counter);
		let handle: thread::JoinHandle<()> =
			thread::spawn(move || {
				let mut num = counter.lock().unwrap();
				*num += 1;
			});
			handles.push(handle);
	}
	println!("\t{:?}", counter);
}

pub fn lib_mutex_1_runner() {
	println!("lib_mutex_1_runner");
	check1();
	check2();
}