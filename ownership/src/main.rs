fn main() {
    let x = 10;

    foo();

    copy();
}

fn foo() {
    let v = vec![1, 2, 3];

    let v2 = v;
}

fn copy() {
    let v = 1;

    let v2 = v;

    println!("v is {}", v);
}
