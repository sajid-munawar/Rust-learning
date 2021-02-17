/*
fn main() {
    let mut counter=0;
    while counter<3 {
        counter=counter+1;        
        println!("Hello, world!");
    };
} */
fn main () {
    let lottery_tickets=[2,55,33,63,27,77];
    let mut counter=0;
    while counter<lottery_tickets.len(){
        println!("{}",lottery_tickets[counter]);
        counter=counter+1;
    }
}