/*
Enums and impl blocks
*/

// enum Weekdays{
//     Sunday,
//     Monday,
//     Tuesday,
//     Wednesday,
//     Thursday,
//     Friday,
//     Saturday,
// }
// fn main(){
//     let _today:Weekdays = Weekdays::Sunday;
// }


enum TravelMode{
    Car(f32),
    Train(f32),
    Flight(f32),
}

impl TravelMode{
    fn reimbursement(&self)-> f32{
        let money:f32 = match self{
            TravelMode::Car(miles) => miles*2.0,
            TravelMode::Train(miles) => miles*5.0,
            TravelMode::Flight(miles) => miles*10.0,
        };
        money
    }
}

fn main(){
    let p1:TravelMode = TravelMode::Car(20.0);
    let new_money:f32 = p1.reimbursement();
    println!("{}",new_money);
}

