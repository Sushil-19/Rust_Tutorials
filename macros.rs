/*
Macros
1. Declerative
2. Procedural
*/

macro_rules! my_macro{
    [] => (
        1+2
    );
    (hello) => {
        println!("Hello should be printed");
    };
    ($no1:expr, $no2:expr, $no3:expr) => {
        ($no1+$no2) - $no3
    }
}

fn main(){
    println!("The answer is {}", my_macro!());
    my_macro!(hello);
    let res:i32 = my_macro!{1,2,3};
    println!("Result is {}", res);
}

