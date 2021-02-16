fn main() {
    println!("Hello, world!");
    square(2,3);

}
fn square(x:u32,y:u32){
    let square_of_x=x*x;
    let square_of_y=y*y;
    println!("square of {} is {}",x,square_of_x);
    println!("square of {} is {}",y,square_of_y);
}