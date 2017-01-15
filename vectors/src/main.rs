fn main() {
    println!("Hello, world!");
    let v = vec![1, 2, 3, 4, 5];   // v: Vec<i32>
    // Vectors store their contents as contiguous arrays of T on the heap.
    // This means, they must know the <T> size at compile time.

    // To get the index of a vector, using '[ ]'
    // And the indices count from 0.
    // For example as below:
    println!("The third element of v is {}", v[2]);
    
    // The index type must be usize, not i32
    let i: usize = 0;
    let j: i32 = 0;

    // Works
    v[i];

    // Do not Works
    // v[j];

    // Access an index that does not exist, the current thread will panic
    // println!("Item 7 is {}", v[6]);
    
    // Handle out-of-bounds error without panicking, using the method like get or get_mut
    // which will return 'None' when given an invaild index.

    let v_out = vec![1, 2, 3];
    match v.get(7) {
        Some(x) => println!("Item 7 is {}", x),
        None => println!("Sorry, this vector is too short")

    }

    // Iterating an vector through its element with for. 
    // There is three version.
    let mut v_iterating = vec![1, 2, 3, 4, 5];

    for i in &v_iterating {
        println!("Version 1, A reference to {}", i);
    }

    for i in &mut v_iterating {
        println!("Version 2, A mutable reference to {}", i);
    }

    for i in v_iterating {
        println!("Version 3, Taking ownership of the vector and its elemment {}", i);
    }

}
