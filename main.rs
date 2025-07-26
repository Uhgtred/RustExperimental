fn main() {
    /*
    This is the main-function of my Rust-Experimental repository.
    This is for learning rust only.
    Nothing meaningful is going to happen in this repository!
     */
    let string_var = "Hello, world!";
    println!("{}", string_var);
    // this is declaring and defining an integer.
    const MY_INT: i8 = 3;
    // the next two lines proof that rust is statically typed
    // let mut my_static_variable = 3;
    // my_static_variable = "Test";
    let mut my_variable_unsigned_int: u8 = 10;
    my_variable_unsigned_int = my_variable_unsigned_int - 10;
    println!("The unsigned integer has been reduced to:\t{}", my_variable_unsigned_int);
    // this is a loop
    for counter in 0..MY_INT {
        println!("This is the result of a loop in the {} run. 2x counter is: {}",counter , counter + counter);
    }

    let mut counter: i8 = 3;
    // This is a while-loop.
    while counter > 0 {
        println!("Running a while-loop. Counter is: {}", counter);
        counter -= 1;
    }
}