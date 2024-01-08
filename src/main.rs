// will push other changes
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn add_address(a: &i32, b: &i32) -> i32 {
    a + b
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;
    vec.push(20);
    vec
}

fn perform_branching() {
    let n = 6;
    if n < 0 {
        println!("It's negative");
    } else if n == 2 {
        println!("It's equal");
    } else if n > 5 {
        println!("It's greater than 5");
    } else {
        println!("Wooow nothing matched");
    }
}

fn do_loop() {
    println!("Let's count till infinity");
    let mut count = 0;
    loop {
        count += 1;
        if count == 6 {
            print!("Six");
            continue;
        }

        if count == 12 {
            print!("Thats enough...");
            break;
        }
    }
}

fn loop_with_returns() -> i32 {
    let mut counter: i32 = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    return result;
}

fn fizz_buzz_with_while() {
    let mut n = 1;
    while n <= 15 {
        if n % 15 == 0 {
            println!("FizzBuzz")
        } else if n % 3 == 0 {
            println!("Fizz")
        } else if n % 5 == 0 {
            println!("Buzz")
        } else {
            println!("{}", n)
        }

        // Increment counter
        n += 1;
    }
}

fn fizz_buzz_with_for() {
    for n in 1..=15 {
        if n % 15 == 0 {
            println!("FizzBuzz")
        } else if n % 3 == 0 {
            println!("Fizz");
        } else if n % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", n);
        }
    }
}

fn for_and_iterators() {
    let mut names = vec!["Pranjal", "Pieter", "Divyansh"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "Pranjal" => "There is a rustacean among us",
            _ => "Hello",
        }
    }

    println!("Names {:?}", names);
}

fn match_in_great_detail() {
    let number = 14;
    println!("Let's match about number {}", number);
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("I am a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Just a normal number"),
    }

    let boolean = false;
    let binary = match boolean {
        true => 1,
        false => 0,
    };

    println!("{} is {}", boolean, binary);
}

fn match_with_tuples() {
    let triple = (17, 10, 36);
    println!("This is a triple {:?}", triple);
    match triple {
        (12, y, z) => println!("First is `12`, `y` is {:?}, and `z` is {:?}", y, z),
        (14, ..) => println!("First is `14`, and rest doesn't matter"),
        (.., 32) => println!("Last is `32`"),
        (16, .., 36) => println!("First is `16` and last is `36`"),
        _ => println!("It doesn't matter what they are"),
    }
}

fn closures_in_rust() {
    let outer_var = 42;
    let closures_anotated = |i: i32| -> i32 { i + outer_var };
    let closures_inferred = |i| i + outer_var;
    // Call the closures now
    println!("Calling the anotated closures {}", closures_anotated(32));
    println!("Calling the inferred closure {}", closures_inferred(32));
}

fn strings_in_rust() {
    let panagram = "the quick brown fox jumps over the lazy dog";
    println!("Panagram: {}", panagram);
    println!("Words in reverse");
    for word in panagram.split_whitespace().rev() {
        println!("> {}", word);
    }

    // Copy chars into a vector, sort and remove duplicates
    let mut chars: Vec<char> = panagram.to_uppercase().chars().collect();
    chars.sort();
    chars.dedup();
    println!("Let's see chars {:?}", chars);
    let long_string = "String literals
    can span multiple lines.
    The linebreak and indentation here ->\
    <- can be escaped too!";
    println!("{}", long_string);
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);
}

fn main() {
    let x = 100;
    let y = 200;
    println!("Hello, world!");
    println!("Let's add {}", add(x, y));
    println!("Let's add with address {}", add_address(&x, &y));
    let mut name: &str = "Pranjal Agnihotri";
    println!("This is my name {}", name);
    name = "hello";
    println!("This is my name {}", name);
    let some_vec = vec![1, 2, 3, 4];
    let new_vec = fill_vec(some_vec);
    println!("Vector after changes {:?}", new_vec);
    perform_branching();
    do_loop();
    println!("\n Loops with returns {}", loop_with_returns());
    fizz_buzz_with_while();
    fizz_buzz_with_for();
    for_and_iterators();
    match_in_great_detail();
    match_with_tuples();
    closures_in_rust();
    strings_in_rust();
}
