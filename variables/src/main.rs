fn main() {
    scalar_types();
    compound_types();
    function_argument(4);
    function_arguments(4, -5);
    statements_and_expressions();
    functions_with_return_values();
    conditions();
    loops();
}

fn loops() {
    let mut x = 0;
    loop {
        println!("looping: {}", x);
        x = x + 1;
        if x >= 5 {
            break;
        }
    }
    x = 0;
    while x < 5 {
        println!("while: {}", x);
        x = x + 1;
    }
    let ary = [10, 20, 30];
    let mut index = 0;
    while index < 3 {
        println!("iterating in slow style -> condition check {}", ary[index]);
        index = index + 1;
    }

    
    println!("Less issues with index bugs... for with iter()");
    for element in ary.iter() {
        println!("fast style without condition {}", element);
    }
    println!("for is better with ranges for counting...");
    for number in (1..4).rev() {
        println!("{}", number);
    }
}

fn conditions() {
    anotation("Conditions");
    bigger_than_four(4);
    bigger_than_four(5);
    println!("Note: evaluation must be bool");
    let number = 3;
    if number != 0 {
        println!("number is not 0");
    }
    diversible_by(4);
    diversible_by(5);
    diversible_by(6);
    let x = if_assignements(4);
    println!("if assignements like in ruby (singletype only), yey ;) {}", x);
}

fn if_assignements(x: i8) -> i32{
    if x == 0 {
        1
    } else {
        x as i32 + 1
    }
}

fn diversible_by(x: i32) {
    if x % 3 == 0 {
        println!("diversible by 3");
    } else if x % 2 == 0 {
        println!("diversible by 2");
    } else {
        println!("not diversible by 3, 2");
    }
}

fn bigger_than_four(x: i32) {
    if x > 4 {
        println!("x is greater than 4");
    } else {
        println!("x is smaller or equal to 4");
    }
}

fn functions_with_return_values() {
    anotation("Functions with return values");
    println!("five, defined by fn: {}", five());
    println!("plus one, calculated by fn: {}", plus_one(five().into()));
}

fn plus_one(x: u32) -> u32 {
    x + 1
}

fn five() -> u8 {
    5
}

fn statements_and_expressions() {
    anotation("Statements and Expressions"); 
    println!("Note: defining a variable like let x = 4; is a statement, they do not evaluate a value");
    println!("Note: functions are statements too");
    println!("Note: Stuff like x = y = 5 like in ruby does not work");
    println!("5 for example is an expression and evaluates to a value -> 5");
    println!("Calling a function or a macro is an expression");
    let x = 4;
    let y = {
        let x = 5;
        x + 1
    };
    println!("Scopes are expressions... {} another x in scope {}", y, x);
    println!("Note: Blocks with curly braces return the last expression without a semicolon");
}

fn function_argument(x: u32) {
    anotation("Function Arguments"); 
    println!("The value of the argument is: {}", x);
    println!("Note: Types must be declared");
}

fn function_arguments(x: u32, y: i32) {
    println!("Multiple Arguments: {}, {}", x, y);
}

fn compound_types() {
    anotation("Compound Types"); 
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

fn scalar_types() {
    anotation("Scalar Types");
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
}

fn anotation(x: &str) {
    println!("");
    println!("++++++++++++++++++ {} ++++++++++++++++++", x);
}
