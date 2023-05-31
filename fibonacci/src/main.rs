use std::io;
fn main() {
    let mut n = String::new();
    
    println!("How many fibonacci numbers do you want to generate?:");
    
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
        
    let n:u32 = n.trim().parse().expect("Failed to read line");
    let mut a=0;
    let mut b=1;
    let mut c=0;
    
    println!("Fibonacci number sequence:");
    
    while c<n{
        println!("{}",a);
        let mut x=a+b;
        a=b;
        b=x;
        c+=1;
    }
}
