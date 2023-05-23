use std::io;

fn ch3_1_variables() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
    let spaces = "   ";
    let spaces = spaces.len();
}

fn ch3_2_data_types() {
    let x = 3.0;
    let y: f32 = 2.0;
    let sum = x + y;
    let diff = x - y;
    let product = x * y;
    let quotient = x / y;
    let floored = 2 / 3;
    let remainder = x % y;

    let t = true;

    let c = 'z';
    let smiley_face = '😀';

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is {y}");
    let a = tup.0;
    let b = tup.1;
    let c = tup.2;

    let a = [3; 5];
    let a = [1, 2, 3, 4, 5];
    let first = a[0];

    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    println!("The value of the array at index {index} is: {element}");

    let message = "The temperature today is:";
    let x = [message; 100];

}

fn ch3_3_functions() {
    fn another_function(x: i32) {
        println!("Another function. The value of x is {x}");
    }

    another_function(5);

    fn print_labeled_measurement(value: i32, unit_label: char) {
        println!("The measurement is: {value}{unit_label}");
    }

    print_labeled_measurement(5, 'h');
    // let x = (let y = 6);  can't do this because let doesn't return a value
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is {y}");

    fn five() -> i32 {
        5
    }

    let x = five();
    println!("The value of x is: {x}");

    fn plus_one(x : i32) -> i32 {
        x + 1
    }
    let x = plus_one(5);
    println!("The value of x is: {x}");


    fn f(x: i32) -> i32 { x + 1 }
    println!("The value of function call f is: {}", f({
        let y = 1;
        y + 1
    }));
}

fn ch3_4_comments() {
    // this is how you do normal comments
    // multiline comments need multiple of these on every line
    let x = 1;
}

fn ch3_5_control_flow() {
    // branches

    let number = 3;

    if number < 5 {
        println!("condition was true");
    }
    else {
        println!("condition was false");
    }

    let number = 6;
    
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    // loops

    // loop {
    //     println!("again");
    // }
    fn first_loop() {
        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter == 10 {
                break counter * 2;
            }
        };
        println!("The result is {result}");
    }
    first_loop();

    fn second_loop() {
        let mut count = 0;
        'counting_up: loop {
            println!("count = {count}");
            let mut remaining = 10;
            loop {
                println!("remaining = {remaining}");
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'counting_up;
                }
                remaining -= 1;
            }
            count += 1;
        }
        println!("End count = {count}");
    }
    second_loop();

    fn while_loop() {
        let mut number = 3;

        while number != 0 {
            println!("{number}!");

            number -= 1;
        }
        println!("LIFTOFF");
    }
    while_loop();
    
    fn for_loop() {
        let a = [10, 20, 30, 40, 50];
    
        for element in a {
            println!("the value is: {element}");
        }
    }
    for_loop();
    
    fn for_loop_2() {
        for number in (1..4).rev() {
            println!("{number}!");
        }
        println!("LIFTOFF");
    }
    for_loop_2();
}


fn main() {
    ch3_1_variables();
    ch3_2_data_types();
    ch3_3_functions();
    ch3_4_comments();
    ch3_5_control_flow();
}
