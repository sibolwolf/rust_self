fn main() {
    println!("Hello, world!");

    let mut x = 5;
    {
        let y = &mut x;
        *y += 1;
    }
    println!("x is {}", x);
}
