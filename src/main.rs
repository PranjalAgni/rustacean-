fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn add_address(a: &i32, b: &i32) -> i32 {
    a + b
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
}
