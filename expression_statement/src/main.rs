fn main() {
    println!("Hello, world!");
    let y= 7;  //this is a statement
    // statement do not return any value while expression returns
    /* let z=(let u=44);
    error: expected expression, found statement (`let`)
    --> src/main.rs:5:12
     |
   5 |     let z=(let u=44);
     |            ^^^^^^^^
     |
     = note: variable declaration using `let` is a statement */
     let number={
         let o=19;
         o+1  //this is expression because it is returning a value
         // that will be save in number
     }; //this whole is statement
     println!("{}",number)
}
