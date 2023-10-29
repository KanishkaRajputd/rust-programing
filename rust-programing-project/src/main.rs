fn main() {
    println!("Hello, world! Khushy");

    //variables 

    // let x = 2;
    // println!("the value of x is {x}")
    // // the value of x is 2

    // re-assignning the value
    // x = 5;
    // println!("the value of x is {x}")
    // // error -  first assignment to `x` help: consider making this binding mutable: `mut x`

    let mut x = 3;
    x = 5;
    
    println!("the value of x is {x}")
    //help: maybe it is overwritten before being read?
    //note: `#[warn(unused_assignments)]` on by default
    // the value of x is 5

}
