/*
Ownerships in Functions
*/
This Video is about ownerships in functions and in primitive

fn main(){
    let v1:Vec<i32>= vec![1,2,3,4];
    takes_ownership(v1.clone());
    println!("{:?}",v1);

    let v2:Vec<i32> = gives_ownersip();
    println!("{:?}",v2);

    let v3:Vec<i32> = gives_and_takes_ownership(v2.clone());
    println!("{:?}",v2);
    println!("{:?}",v3);

    let num:i32 = 10;
    fun(num);
    println!("{}",num);
}

fn takes_ownership(v: Vec<i32>){
    println!("{:?}",v);
}

fn gives_ownersip()-> Vec<i32>{
    vec![5,6,7]
}

fn gives_and_takes_ownership(mut v:Vec<i32>)->Vec<i32>{
    v.push(10);
    v
}

fn fun(mut n:i32){
    println!("{}",n);
    n=20;
}