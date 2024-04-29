/*
Structures.
*/

struct Car{
    owner: String,
    price: u32,
    fuel: f32,
    milage: f32,
}

fn main(){
    let mut newcar:Car = Car{
        owner: String::from("RamLal"),
        price: 2_000,
        fuel: 21.5,
        milage: 10.2,
    };

    let owner1:String = newcar.owner;
    println!("{}", owner1);
    newcar.owner = "Ganpat".to_string();
    println!("{}",newcar.owner);

    let mut mycar:Car = Car{
        owner: String::from("Ram"),
        ..newcar
    };
    println!("{}", mycar.owner);
    println!("{}", mycar.price);

    let tuple1:(i32,i32) = (1,2);
    let tuple2:(i32,i32,i32) = (3,4,5);

    struct Tuple1 (i32,i32);
    struct Tuple2 (i32,i32,i32);

    let t1:Tuple1 = Tuple1(1,2);

    //unit struct
    struct UNIT;
}