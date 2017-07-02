fn main() {
    let s = String::from("Hello");

    takes_ownership(s);

    // Cannot use s any more - ownership has moved to the takes_ownership function
    // and s was droped
    // after function exited
    // println!("{}", s);

    let x = 5;

    makes_copy(x);

    // i32 value was copied when passed to makes_copy function
    println!("{}", x);

    let s1 = gives_ownership();
    println!("{}", s1);

    let s2 = String::from("Hello");

    let s3 = takes_and_gives_back(s2);

    println!("{}", s3);

    let len = calculate_length(s3);

    println!("{}", len);

    // cannot use s3 anymore - ownership moved to calculate_length function
    // println!("{}", s3);

    let s4 = String::from("Hello");
    let (s4, len_with_ownership) = calculate_length_and_give_back_ownership(s4);

    println!("{}, {}", s4, len_with_ownership);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("Hello");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> usize {
    s.len()
}

fn calculate_length_and_give_back_ownership(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

