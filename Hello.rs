/*
variables
mutability
scopes
Shadowing
constants
*/
// This is single lined comment

//#![allow(unused_assignments)]
fn main(){
    // let x: i32 = 10;
    // x=20;
    
    //let mut y: i32 = 10;
    //y=30;
    {
        // let z:i32= 100;
        //println!("{}",y);
    }
    // println!("{}",z);

    let a: i32=10;
    println!("{}",a);
    let a: f32=10.50;
    println!("{}",a);

    //const I_AM_CONST : i32 = 200;
    // I_AM_CONST = 300;
    //println!("This is a constant: {}", I_AM_CONST);
}