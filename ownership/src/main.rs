fn main() {
    let s = "hello";
    println!("s belongs to main, {}", s);
    fn_scope(s);

    let mut s = String::from("hello"); // eg. user input -> from
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`
    {
        let s = String::from("hello");
        println!("end of scope for {}", s);
    }

    stack_variables();
    heap_variables();
    takes_ownership(String::from("hi"));
    makes_copy(4);
    let string = String::from("hey");
    let other_string = takes_ownership_and_gives_back(string);
    println!("we got back the ownership! {}", other_string);

    println!("");
    println!("Keep the ownership with a reference, yey;)");
    let string = String::from("crap");
    println!("we could keep the ownership and go on with: string, len {}, {}", i_do_not_want_the_ownership(&string), string);
    println!("Note: Keep in mind, we cannot change something we do not own");
    let mut s = String::from("hell");
    println!("before: {}", s);
    change(&mut s);
    println!("after: {}", s);

    println!("");
    println!("Note: we cannot pass the reference to 2 parties, if mutable in the same scope -> solves many issues with parallel accessing, eg.  cannot borrow `s` as mutable more than once at a time");

    different_scopes();

    iterate_string();
}

fn iterate_string() {
    let st = String::from("hello you");
    let pos = first_word(&st);
    println!("position of first space is: {}", pos);
    let first_word = better_first_word(&st);
    println!("first_word: {}", first_word);
}

fn better_first_word(s: &String) -> String {
    let bytes = s.as_bytes();
    let mut pos = s.len();
    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            pos = index;
            break;
        }
    }
    String::from(&s[0..pos])
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return index;
        }
    }

    s.len()
}

fn different_scopes() {
    println!("Possible in different scopes...");
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        r1.push_str(" r1 ");
        println!(" r1: {}", r1);
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let r2 = &mut s;
    r2.push_str(" r2");
    println!(" r2: {}", r2);
    println!("Note: Same issue with mixing mutable and immutable references -> Problem");
    println!("Note: Dangling references are not possible -> compile time check, means, references must always be valid!");
}

fn change(x: &mut String) {
    x.push_str(", world");
}

fn i_do_not_want_the_ownership(x: &String) -> u16 {
    x.len() as u16
}

fn takes_ownership_and_gives_back(some_string: String) -> String {
    println!("takes ownership, is not longer valid outside of scope ");
    println!("{}", some_string);
    some_string
}

fn takes_ownership(some_string: String) { // some_string comes into scope.
    println!("takes ownership, is not longer valid outside of scope ");
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope.
    println!("makes a copy: ");
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn heap_variables() {
    let x = String::from("hi");
    let y = x;
    println!("heap variables are different... {},", y);
    println!("Note: strings consist of a pointer to the heap, a length, and a capacity");
    println!("x is not available anymore (moved to y), it goes out of scope -> double free issues solved like that");
    cloning();
}

fn cloning() {
    let x = String::from("hi");
    let y = x.clone();

    println!("cloning is possible, x, y: {}, {}", x,y);
    println!("Note: more about Copy and Drop on Stack -> appendix");
}

fn stack_variables() {
    let x = 1;
    let y = x + 1;
    println!("x and y assigned, {}, {}", x, y);
}

fn fn_scope(x: &str) {
    println!("still main scope: {}", x);
}
