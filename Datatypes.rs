/*
Primitive Datatypes:
Integer
Float
Char 
Bool
*/

fn main(){

    let _signed_int : i8 = 5;    // Signed integer -> i8, i16, i32, i64, i128
    let _unsigned_int : u8 = 10; // Unsigned integer -> u8, u16, u32, u64, u128

    let _f_var : f32 = 10.5;     // Float -> f32, f64

    let _c_var : char = 'a';     // character -> must be in quotes

    let _bool_val : bool = true; // boolean -> true/false

    // Type Aliasing

    type Age = u16;
    let _age : Age = 32;

    // Type conversion
    let a: u32 = 10;
    let _b: f32 = a as f32;
}