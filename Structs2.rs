/*
Structures and impl blocks
*/


struct Car{
    owner: String,
    price: u32,
    fuel: f32,
    milage: f32,
}

impl Car{
    fn installment(&self)-> u32{
        100
    }

    fn selling_price(&mut self) -> u32{
        self.price + Car::installment(self)
    }

    fn display(&self){
        println!("{} {} {} {}", self.owner, self.price, self.fuel, self.milage);
    }

    fn refuel(&mut self, liter:f32){
        self.fuel += liter;
    }

    fn sell(self) -> Self{
        self
    }

    fn new(oname:String, price:u32, milage:f32) -> Car {
        let car:Car = Car{
            owner: oname,
            price: price,
            fuel: 0.0,
            milage: milage,
        };
        car
    }

}


fn main(){
    let mut newcar:Car = Car{
        owner: String::from("RamLal"),
        price: 2_000,
        fuel: 21.5,
        milage: 10.2,
    };

    newcar.display();
    newcar.refuel(10.20);
    newcar.display();
    let new_price = newcar.selling_price();
    println!("{}",new_price);
    let chnage_ower = newcar.sell();
    //newcar.refuel(20.0);

    let another_car = Car::new("Maruti".to_string(), 10, 100.0);
    another_car.display();
}