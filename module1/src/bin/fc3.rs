// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively
//
// Notes:
// * Use a variable set to any integer
// * Use a match expression to determine which message to display
// * Use an underscore (_) to match on any value

fn main() {

    let a = 33;
    match a {
        1=> println!("Not the default value"),
        2=> println!("Not the default value"),
        3=> println!("Not the default value"),
        4=> println!("Not the default value"),
        33=> println!("Value Matched!"),
        _=> println!("This is the default value"),
    }
}


