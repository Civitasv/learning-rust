const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // const variables can never be changed

fn main() {
    variables();
    datatypes();
    control_flow();
}

fn variables() {
    // by default, variables are immutable
    let x = 5; // x cannot change
    println!("The value of x: {}", x);
    let x = 6; // but you can shadow it
    println!("The value of x: {}", x);
    let mut y = 7; // make a variable that can change
    println!("The value of y: {}", y);
    y = 5; // it's ok to change it
    println!("The value of y: {}", y);
    // another example of shadowing
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}
fn datatypes() {
    // Scalar types
    let x: i8 = 2;
    let y: i16 = 4;
    let z: i32 = 8;
    let a: i64 = 2;
    let b: i128 = 3;
    let t: bool = true;
    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';
    let tup: (i32, f64, u8) = (500, 6.4, 1); // tuple

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup; // destructure it
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    let a = [1, 2, 3, 4, 5]; // array

    let a: [i32; 5] = [1, 2, 3, 4, 5]; // [type; number]
    let a = [3; 5]; // [value; number]
}

fn functions() -> i32 {
    fn print_labeled_measurement(value: i32, unit_label: char) {
        println!("The measurement is: {}{}", value, unit_label);
    }
    // Statements are instructions that perform some action and do not return a value.
    // Expressions evaluate to a resulting value.
    let x = 6; // statement
               // ley y = (let x =6); // error
    let y = {
        let x = 3;
        x + 1 // expressions do not include ending semicolons
    };
    fn five() -> i32 {
        5 // return 5
    }
    return 2;
}

fn control_flow() {
    let number = 3;
    // condition expression must be bool
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    // multiple conditions
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

    // with let
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
    // forever
    loop {
        println!("again!");
        break;
    }
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
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
    println!("End count = {}", count);

    // loop can return value
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
    // while
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // for
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
