fn main() {
    const MAX_POINTS: u32 = 100_000;
    println!("max points u32 constant is: {}", MAX_POINTS);
    let mut x = 5;
    println!("the value of x is: {}", x);
    x = 4;
    println!("the value of x is: {}", x);
    let x = x + 1;
    println!("shadow x increased by one is: {}", x);
    let x = x * 2;
    println!("shadow x multiplied by two is: {}", x);

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("parsed string to: {}", guess);

    let x = 2.0; // f64
    println!("floating f64: {}", x);

    let y: f32 = 2.0; // f64
    println!("floating f32: {}", y);

    let sum = 4 + 6;
    println!("the sum of 4 and 6 is: {}", sum);

    let difference = 4 - 6;
    println!("the substraction of 4 and 6 is: {}", difference);

    let product = 4 * 6;
    println!("the product of 4 and 6 is: {}", product);

    let quotient = 4.0 / 6.1;
    println!("the division of 4 and 6.1 is: {}", quotient);

    let remainder = 42 % 5;
    println!("the remainder of 42 % 5 is: {}", remainder);

    let t = true;
    let f: bool = false;
    println!("Most languages have {} and {} as bool types", t, f);

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("Unicode Characters, YEY;): {}, {}, {}", c, z, heart_eyed_cat);

    println!("+++++++++++++ Compound Types ++++++++++++++");
    let tup: (i32, f64, u8) = (500, 3.14, 2);
    let (_x,y,_w) = tup;
    println!("Ruby's assign array, Python's Tuples -> destruction, yey, {}", y);
    let five_hundred = tup.0;
    let pi = tup.1;
    let two = tup.2;
    println!("Tuples rock: {}, {}, {}", five_hundred, pi, two);

    let array = [1,2,4,5];
    println!("first: {}", array[0]);
    println!("last: {}", array[3]);
    println!("Note: Index out of bounds exists in Rust too ;)");
    


}
