fn main() {

 let student:(u32,u32,u32,char,f64)=(22,9,35,'a',2.5);
println!("{}",student.0);
println!("{}",student.1);
println!("{}",student.2);
println!("{}",student.3);
println!("{}",student.4);
println!("destructering");
let (a,b,c,d,e)=student;
println!("{}",a);
println!("{}",b);
println!("{}",c);
println!("{}",d);
println!("{}",e);
println!("{}","end");
}
 