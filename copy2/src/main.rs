fn main() {
    let a = true;

    let _y = change_truth(a);
    println!("a is {}", a);
}

fn change_truth(x: bool) -> bool {
    !x
}