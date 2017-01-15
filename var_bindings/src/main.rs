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

    let (x, y, z) = (1, 2, 3);
    println!("x is {}", x);
    let tuple_single = (0,);   //This is a single element tuple 
    let var_single = (0);   //This is a single variable bindings
    println!("var_single is {}", var_single);

    let tuple = (1, 2, 3);
    let x = tuple.0;
    let y = tuple.1;
    let z = tuple.2;
    println!("x is {}", x);

    let x: fn(i32) -> i32 = foo;
    let y = x(34);
    println!("y is {}", y);

    // Line comments are anything after '//' and extend to the end of the line.
    // Put a space between the '//' and the comments so that it is more readable.

    /// Doc comments using /// instead of //, and support makedown notation inside
    /// Adds one to the number given.
    ///
    /// # Examples
    ///
    /// ```
    /// let five = 5;
    ///
    /// assert_eq!(6, add_one(5));
    /// # fn add_one(x: i32) -> i32 {
    /// #     x + 1
    /// # }
    /// ```
    fn plus_one(x: i32) -> i32 {
        x + 1
    }

    let x = 7;
    if x == 5 {
        println!("x is five!");
    } else if x == 6 {
        println!("x is six!");
    } else {
        println!("x is not five or six");
    }

    let y = if x == 5 {
        10
    } else {
        15
    };   // y: i32

    println!("y is {}", y);

    // Or let y = if x == 5 { 10 } else { 15 }; // y: i32

    // Rust provide 3 kinds of iterative activity: loop, while, for.

    let mut x = 5;  // mut x: i32
    let mut done = false;   // mut done: bool

    while !done {
        x += x - 3;
        println!("x iterative value is {}", x);

        if x % 5 == 0 {
            done = true;
        }
    }

    // The 'for' loop is used for a particular number of times

    for x in 0..10 {
        println!("the 'for' loop for x is {}", x);   // x: i32
    }

    // The enumerate function used for keeping track how many times of the 'for' loop execute.

    for (index, value) in (5..10).enumerate() {
        println!("index is {} and value is {}", index, value);
    }

    let lines_all = "hello\nworld".lines();

    for (linenumber, line) in lines_all.enumerate() {
        println!("{}, {}", linenumber, line);
    }

    // Use break or continue to stop the interation

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

fn foo(x: i32) -> i32 { x }