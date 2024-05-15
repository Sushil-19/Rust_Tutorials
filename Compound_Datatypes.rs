/*
Compound datatypes
String and &str
Arrays
Vectors
Touple
Empty tuple
*/

fn main(){
    let _slice_string : &str = "This string is immutable";
    let mut _normal_string : String = String::from("This is a mutable string");

    let mut _arr : [i32; 5] = [5,6,7,8,9];
    let _num = _arr[3];

    let _arr2 : [i32;5]= [10; 5];
    println!("{:?}",_arr2);
    let _vec: Vec<i32> = vec![10,11,12,13,14];

    let _myinfo :(&str, i32, &str, i32) = ("Salary", 4000, "Age", 40);
    let _salary_val = _myinfo.1;

}