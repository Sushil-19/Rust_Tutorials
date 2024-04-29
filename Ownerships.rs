/*
1. The variable has only one owner.
2. The variable can be cloned so that the other var can have its own copy.
*/

fn main(){
    let s1:String = String::from("RamLal");
    let _s2:String = s1.clone();
    println!("{}",s1);


    let x:i32= 10;
    let _y:i32= x;
    println!("{}",x);
}