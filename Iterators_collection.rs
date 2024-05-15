/*
Iterators on collections
iter(), iter_mut(), into_iter()
*/

use std::collections::HashMap;

fn main(){
    let v1: Vec<i32> = vec![10,20,30,40];
    let mut v1_iter = v1.iter();

    // println!("{:?}", v1_iter.next());
    // println!("{:?}", v1_iter.next());
    // println!("{:?}", v1_iter.next());
    // println!("{:?}", v1_iter.next());
    // println!("{:?}", v1_iter.next());

    for i in &v1{
        println!("{:?}", i);
    }
    println!("{:?}", v1_iter.next());

    let mut m1: HashMap<String, i32> = HashMap::new();
    m1.insert("RamLal".to_string(), 1);
    m1.insert("ShamLal".to_string(), 2);
    m1.insert("HariLal".to_string(), 3);

    for (name, no) in m1{
        println!("{:?}, {:?}", name, no);
    }
}
