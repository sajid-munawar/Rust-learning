fn main() {
    let mut counter=2;
    let mut val_count=0;
    loop {
        val_count=val_count+1;
        counter=counter+1;
        println!("Hello world!");
        if (counter==5){
            break 
        }
    };
    println!("{}",val_count);
    /*
    in this program we are checking how to create and break a loop
    how to count iterations */

}
