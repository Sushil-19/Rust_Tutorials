/*
Functions,
CodeBlocks
*/

fn main(){
    anotherfun("Hello this is another function");
    //println!("{}",operation(20,10));
    let t:(i32,i32,i32,i32) = operation(20,10);
    println!("{} {} {} {}", t.0,t.1,t.2,t.3);

    let _full_name:String = {
        let name:&str = "RamLal";
        let surname:&str = "Firangi"; 
        format!("{} {}", name, surname)
    };

    println!("{}", _full_name);
}

fn anotherfun(st:&str){
    println!("{}", st);
}

fn operation(num1:i32, num2:i32) -> (i32,i32,i32,i32){
    println!("Operating");
    (num1+num2, num1-num2, num1*num2, num1/num2)
}