fn main() {
    println!("Hello, world!");
    let mut x: i32 = 5;
    x = 10;

    print_num(5);

    add_num(4, 6);

    let x1: i32;
    x1 = add_one(7);

    let f: fn(i32) -> i32 = add_one;
    let ff = add_one;   
    let x2 = f(6);
    let x3 = ff(8);

    let x_bool = true;
    let y_bool: bool = false;
    println!("x_bool's value: {}", x_bool);

    let x_char = 'H';
    println!("x_char's value: {}", x_char);
    
    let a = [1, 2, 3];   //a: [i32; 3]
    let mut m = [1, 2, 3];   //m: [i32; 3]
    let aa = [0; 20];   //aa is an array with 0*20
    println!("aa has {} elements", a.len());
    println!("m's second value is {}", m[1]);

    let names = ["Xiaoming", "Xiaohong", "Xiaohua"];   //names: [&str; 3]
    println!("The second name is {}", names[1]);

    let sum = [0, 1, 2, 3];
    let sum_complete = &sum[..];   //A slice containing all of the elements
    let sum_middle = &sum[1..4];   //A slice only contain element from 1 to 3
    println!("sum_complete's length is {}", sum_complete.len());
    println!("sum_middle's length is {}", sum_middle.len());
}

fn print_num(x: i32){
    println!("x is {}", x);
}

fn add_num(x: i32, y: i32){
    println!("sum is {}", x + y);
}

fn add_one(x: i32) -> i32{
    x + 1
}

