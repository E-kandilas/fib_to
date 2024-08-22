/////////////
// Fib to //
////////////

// use std::io;

// fn fib_to(fib_num: i32) {
//     if fib_num < 0 {
//         println!("Please enter a positive number");
//         return;
//     }

//     if fib_num == 0 {
//         println!("0");
//         return;
//     }

//     if fib_num == 1 {
//         println!("0\n1");
//         return;
//     }

//     let mut current = 0;
//     let mut next = 1;
// while current <= fib_num {
//     println!("{}", current);
//     let new_next = current + next;
//     current = next;
//     next = new_next;
//     }
// }


// fn main() {
//     let mut user_input = String::new();
//     io::stdin().read_line(&mut user_input).unwrap();

//     let fib_input: u32 = user_input.trim().parse().expect("Please enter a number");

//     fib_to(fib_input.try_into().unwrap());
// }


///////////////
// Baby Calc //
//////////////
use std::io;

fn hit_the_nums() {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();

    if user_input.trim() == "exit" {
        kill_the_nums();
    }

    let mut split_input = user_input.split_whitespace();
    let num1: i32 = split_input.next().unwrap().parse().unwrap();
    let operator = split_input.next().unwrap();
    let num2: i32 = split_input.next().unwrap().parse().unwrap();


    match operator {
        "+" => {
            let result = num1 + num2;
            println!("Result: {}", result);
        }
        "-" => {
            let result = num1 - num2;
            println!("Result: {}", result);
        }
        "*" => {
            let result = num1 * num2;
            println!("Result: {}", result);
        }
        "/" => {
            if num2 == 0 {
                println!("Error: Division by zero");
            } else {
                let result = num1 / num2;
                println!("Result: {}", result);
            }
        }
        _ => {
            println!("Error: Invalid operator");
        }
    }
}

fn kill_the_nums() {
    std::process::exit(0);
}

fn main() {
    while true {
        hit_the_nums();
    }
}
