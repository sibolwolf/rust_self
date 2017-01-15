fn main() {
    println!("Hello, world!");

    let mut x = 5;
    loop {
        x += x - 3;

        println!("x is {}", x);

        if x % 5 == 0 { break; }
    }

    for y in 0..10 {
        if y % 2 == 0 { continue; }
        println!("y is {}", y);
    }

    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 { continue 'outer; }
            if y % 2 == 0 { continue 'inner; }
            println!("x: {}, y: {}", x, y);
        }
    }
}
