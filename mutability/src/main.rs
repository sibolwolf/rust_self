fn main() {
    println!("Hello, world!");
    let mut x = 5;
    {
        let y = &mut x;
        *y = 6;
    }

    println!("x is {}", x);

    foo();
}

fn foo() {
    let mut xx = 8;
    let mut zz = 10;
    println!("xx origin value is {}", xx);
    println!("zz origin value is {}", zz);

    {
        let mut yy = &mut xx;
        *yy = 88;
    
        println!("xx is {} which has been changed by reference yy", *yy);

        yy = &mut zz;
        println!("yy is {} which has been changed to a new mutable reference zz", yy);
    }

    println!("xx is {} which has been changed by yy before", xx);
    
}

fn struct_example() {
    struct Point{
        x: i32,
        y: i32,
    }

    let mut a = Point {x: 5, y: 6};

    a.x = 10;
    a.y = 20;

    let b = Point {x: 5, y: 6};
    
    b.x = 200
}