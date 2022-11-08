// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result
fn display_result(c:i32){
    println!("{}",c);
}

fn add_two_int(a: i32, b: i32) -> i32 {
    return a+b;
}
fn main() {
    let a = 2;
    let b = 4;
    let c  = add_two_int(a, b);
    display_result(c);

}
