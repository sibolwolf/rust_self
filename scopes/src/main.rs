fn main() {
    println!("Hello, world!");
    let mut x = 5;
    
    let y = &x;
    //*y += 1;
    
    println!("y is {}", y);
    println!("*y is {}", *y);
    println!("x is {}", x);
}
