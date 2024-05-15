/*
Borrowing,

1. At a time you can have only one mutable referance OR any number of immutable referances.
2. Referances must always be valid
*/

fn main(){
    let mut v:Vec<i32> = vec![1,2,3,4];
    let vref1:&mut Vec<i32> = &mut v;
    //let vref2:&Vec<i32> = &v;
    println!("{:?}", vref1);

    let vref3:&Vec<i32> = {
        let v2:Vec<i32> = vec![7,8,9];
        &v2
    };
}