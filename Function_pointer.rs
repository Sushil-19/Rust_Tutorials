/*
Function Pointers
*/

struct User{
    name: String,
    age: u32,
    salary:f32,
}

// fn validateUsr(name:&str)->bool{
//     name.len() != 0
// }

fn validateUsr(
    name:&str, 
    age:u32, 
    validate: fn(&str)->bool, 
    validate_advance: fn(u32)->bool
)->bool
{
    validate(&name) && validate_advance(age)
}

fn validate(name:&str)-> bool{
    name.len()!=0
}

fn validate_advance(age:u32) -> bool{
    age>10
}

fn main(){
    let p1:User = User{
        name:String::from("RamLal"),
        age:30,
        salary:1000.00,
    };

    //println!("{}", validateUsr(&p1.name));
    let banned_usr:String = String::from("banned user");
    // let validate = |name:&str| {
    //     let b_usr:String = banned_usr;
    //     name.len()!=0 && name!= b_usr
    // };
    // println!("{}", banned_usr);
    // let validate_advance = |age:u32| {age>10};
    // println!("{}", validate(&p1.name));
    println!("{}", validateUsr(
        &p1.name,
        p1.age,
        validate,
        validate_advance
    ));
}

