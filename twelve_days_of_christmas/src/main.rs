fn main(){
    let mut num =1;
    let mut chars = ["n","Twelve drummers drumming","Eleven pipers piping","Ten lords a-leaping","Nine ladies dancing","Eight maids a-milking","Seven swans a-swimming","Six geese a-laying","Five golden rings","Four calling birds","Three french hens","Two turtle doves, and","A partridge in a pear tree"];
    let mut x =13;
    let mut y =12;
    while num<13{
        println!("On the {} day of Christmas, my true love sent to me",num);
        for n in y..x{
            println!("{}",chars[n]);
        }
        y-=1;
        num+=1;
        println!(" ")
    }
}
