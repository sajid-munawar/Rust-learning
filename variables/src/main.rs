fn main() {
    let age = 50;
    age = 55  //error: cannot assign twice to immutable variable
    // so we will use mut keyword
    let mut temperature =35;
    temperature=40;
    println!("{}",temperature) ;
}

// println!("{}",temperature) 
