fn main() {
    // println!("Hello, world!");
    let (value,value_1)=square(2,5.5);
    println!("{},{}",value,value_1);
}
fn square(x:u32,y:f64) -> (u32,f64) {
    let result=x*x;
    let result_1=y*y;
    //result  //there should not be a semicolon here
    //above is for one value if we need two we pass a tuple
    (result,result_1)
    // println!("{},{}",result,result_1);
}
