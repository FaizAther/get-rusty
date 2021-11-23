
mod lib;

fn main() {
    let a: [i32; 3]= [1,2,3];
    
    let v:Vec<i32> = Vec::from([1,2,3,4]);
    let v = vec![1,2,3,4]; // same as above
    //v.push(1);
    let v_2 = &v[2]; // pointer to second indexed i.e. 3

    match v.get(20) {
        Some(third) => println!("thrid {}", third),
        None => println!("out of bound")
    }

    match v.get(2) {
        Some(third) => println!("thrid {}", third),
        None => println!("out of bound")
    }
    
    let mut v_m: Vec<i32> = Vec::new();
    
    v_m.push(1);

    println!("mut vec pop: {}", v_m.pop().unwrap());
    //println!("mut vec pop: {}", v_m.pop().unwrap()); // unwrapping a none: runtime error


    let mut vm = vec![1,2,3,4,5,6,7,8];
    println!("vec size:{}", vm.len());
    for i in vm {
        println!("{}", i);
    };
    //  println!("vec size:{}", vm.len()); // not allowed: borrow err
    let mut vmm = vec![1,2,3,4,5,6];
    for i in &mut vmm {
        *i += 50;
    };
    for i in &vmm {
        println!("{}", i);
    };
    println!("vec size:{}", vmm.len());

    lib::runner();

    let v_err = &v[20]; // out of bound : run time error
}
