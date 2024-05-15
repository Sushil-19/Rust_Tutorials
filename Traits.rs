/*
Traits in Rust
*/

struct Squre{
    len:u32,
    color:String,
    line_wid:u32,
}

struct Rectangle{
    len:u32,
    wid:u32,
    color:String,
    line_wid:u32,
}

trait Shape{
    fn area(&self) -> u32{
        println!("The method is not implemented in impl block, returning default");
        0
    }
}

// impl Squre{
//     fn area(&self)-> u32{
//         self.len*self.len
//     }
// }

// impl Rectangle{
//     fn area_of_rec(&self){
//         println!("{}",self.len*self.wid);
//     }
// }

impl Shape for Squre{
    fn area(&self) -> u32{
        self.len*self.len
    }
}

impl Shape for Rectangle{
    fn area(&self) -> u32{
        self.len*self.wid
    }
}


fn main(){
    let s1:Squre = Squre{
        len:10,
        color:String::from("Black"),
        line_wid:5,
    };

    let r1:Rectangle = Rectangle{
        len:10,
        wid:20,
        color:String::from("Black"),
        line_wid:5,
    };

    println!("{}",s1.area());
    println!("{}",r1.area());
}