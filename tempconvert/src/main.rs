use std::io;
fn main(){
    let mut num = String::new();
    
    println!("Enter the number to convert:");
    
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");
    
    let num: i32 = num.trim().parse().expect("Failed to read line");
    
    let mut f_c = String::new();
    
    println!("Convert to Fahrenheit or Celsius? (f/c)");
    
    io::stdin()
        .read_line(&mut f_c)
        .expect("Failed to read line");
    
    let f_c = f_c.trim();
    
    if f_c.eq("f"){
        let mut f =  (num *9/5) + 32;
        println!("Temperature in F: {}",f);
    } else if f_c.eq("c") { 
        let mut c =  (num -32)*5/9;
        println!("Temperature in C: {}",c);
    }else {
        println!("Failed to read line")
    }
}
