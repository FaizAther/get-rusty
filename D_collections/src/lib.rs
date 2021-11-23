enum Cell {
    Int(i32),
    Float(f64),
    Text(String)
}

pub fn runner() {
    let row = vec! [
        Cell::Int(3),
        Cell::Float(4.333),
        Cell::Text(String::from("Testing"))
    ];
    
    match &row[1] {
        Cell::Int(i) => println!("got an int cell: {}", i),
        _ => println!("something else")
    }

    match &row[0] {
        Cell::Int(i) => println!("got an int cell: {}", i),
        _ => println!("something else")
    } 
}