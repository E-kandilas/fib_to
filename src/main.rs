use std::io;

fn fib_to(fib_num: i32) {
    if fib_num < 0 {
        println!("Please enter a positive number");
        return;
    }

    if fib_num == 0 {
        println!("0");
        return;
    }

    if fib_num == 1 {
        println!("0\n1");
        return;
    }

    let mut current = 0;
    let mut next = 1;
while current <= fib_num {
    println!("{}", current);
    let new_next = current + next;
    current = next;
    next = new_next;
    }
}


fn main() {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();

    let fib_input: u32 = user_input.trim().parse().expect("Please enter a number");

    fib_to(fib_input.try_into().unwrap());
}
