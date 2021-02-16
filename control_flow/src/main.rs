fn main() {
    // i am using in char it can be int as well
    let number="-6";
    if number<"0" {
        println!("This is negative num");
    }
    else if number == "0" {
        println!("The number is zero");
    }
    else {
        println!("Positive number");
    }



   let even = false; 
   let number = {
       if even {
           6
       }
       else {
           7
       }
    };
    println!("{}",number);
}