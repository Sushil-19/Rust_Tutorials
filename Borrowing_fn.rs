/*
Borrwing in functions.

1. At a time you can have only one mutable referance OR any number of immutable referances.
2. Referances must always be valid
*/

fn main(){
    let mut vec1:Vec<i32> = vec![1,2,3,4];
    let vref1:&Vec<i32> = &vec1;
    printit(vref1);
    println!("{:?}",vec1);
    let vref2:&mut Vec<i32> = &mut vec1;
    let vref3:&Vec<i32> = addnum(vref2);
    println!("{:?}", vref3);
    println!("{:?}",vec1);

    let v4:Vec<i32> = retnewvec();
    println!("{:?}",v4);
    
}

fn printit(vref:&Vec<i32>){
    println!("{:?}", vref);
}

fn addnum(vref:&mut Vec<i32>)->&Vec<i32>{
    vref.push(10);
    vref
}

fn retnewvec() -> Vec<i32>{
    let v:Vec<i32> = vec![9,8,7];
    v
}