fn main() {
    println!("Hello, world!");
    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];

    // let (v11, v22, answer) = foo(v1, v2);
    let answer = foo2(&v1, &v2);

    // we can use v1 and v2 here
    println!("v1[1] is {}", v1[1]);
    println!("v2[1] is {}", v2[1]);
    println!("answer is {}", answer);
}

fn foo(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
    // do stuff with v1 and v2

    // ahd back ownership, and the result of our function
    (v1, v2, 42)
}

fn foo2(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    // do stuff with v1 and v2

    // return the answer
    let s1 = sum_vec(v1);
    let s2 = sum_vec(v2);
    return s1 + s2;
}

// Ignor how 'fold' works, the point here is that an immutable reference is borrowed.
fn sum_vec(v: &Vec<i32>) -> i32 {
    return v.iter().fold(0, |a, &b| a + b);
}