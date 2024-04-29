/*
Derefrencing of variables.

-> Transfer of ownership (Copying).
*/

fn main(){
    let mut data:i32 = 10;
    let dataref:&mut i32 = &mut data;
    let dref:i32 = *dataref;
    *dataref = 50;
    println!("{}, {}",data,dref);

    let mut vec:Vec<i32> = vec![1,2,3];
    let vref1:&Vec<i32> = &vec;
    let vref2:&Vec<i32> = vref1;
    let vref3:&mut Vec<i32> = &mut vec;
    println!("{:?}", vref3);
}