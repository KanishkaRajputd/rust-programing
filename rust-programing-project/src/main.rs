fn main() {
    println!("Hello, world! Khushy");

    //variables 

    // let x = 2;
    // println!("the value of x is {x}")
    // // the value of x is 2

    // re-assignning the value>>>>>>>>>>>>>
    // x = 5;
    // println!("the value of x is {x}")
    // // error -  first assignment to `x` help: consider making this binding mutable: `mut x`

    // let mut x = 3;
    // x = 8 + 2;
    // assert_eq!(x,10);

    // println!("the value of x is {x}")
    //help: maybe it is overwritten before being read?
    //note: `#[warn(unused_assignments)]` on by default
    // the value of x is 5

    // block scope>>>>>>>>>>>>>>
    // let x = "hello";
    // {
    //     let y = "world";
    //     // println!("the value of x is {} and value of y is {}", x, y)
    // }
    // println!("the value of x is {} and value of y is {}", x, y)
//       println!("the value of x is {} and value of y is {}", x, y)                                                             ^
// help: the binding `y` is available in a different scope in the same function
//   --> src\main.rs:27:13


//function initilizing
// initilizing_x(7);

// multiple initilization >>>>>>>>
// let (a,b,c) = (2,3,4)
// println!("the value of a is {}.", a)

        //               ^ help: add `;` here
        //  println!("the value of a is {}.", a)
        // ------- unexpected token

        
// let (a,b,c) = (2,3,4);

// println!("the value of a is {}.", a);
// println!("the value of b is {}.", b);
// println!("the value of a is {}.", c);


//destrucuring >>>
//  we don't have to do anything with last two values and I just wanna put the first one ;
//  or we don't have to do anything with first two values and I just wanna put the last one;

let (a,..) = (2,3,4);
let (..,b) = (2,3,4);
let [..,c] = [2,3,4,5];

println!("the value of a is {}.", a);
println!("the value of b is {}.", b);
println!("the value of c is {}.", c);
 
assert_eq!([a,b,c] , [2,4,5]);
println!("Success!");


// data types >>>

// integar >>
//           signed variable (positive or negative)     unsigned variables(positive value only)  
// 8 -bit                i8                                                u8
// 16 -bit               i16                                               u16
// 32 -bit               i32                                               u32
// 64 -bit               i64                                               u64
// 128 -bit              i128                                              u128
// arc                   isize                                             usize

// boolean >> true, false
//float f32 , f64 (6.00)
// characters z,Z,😊
//tupples (x,y,z)
//array [x,Sy,z]


}

// fn initilizing_x(x: i32){
//         println!("the value of x is {}.", x)
// }