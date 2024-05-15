/*
SuperTraits.
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

trait Draw{
    fn draw_shape(&self);
}

trait Shape: Draw + SomeTrait{
    fn area(&self) -> u32{
        println!("The method is not implemented in impl block, returning default");
        0
    }
}

trait SomeTrait{}
impl SomeTrait for Rectangle{}
impl SomeTrait for Squre{}

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

fn shape_prop<T>(object: T) 
where 
T:Shape{
    println!("This is in shape_prop function.");
    println!("{}",object.area());
}

fn return_shape()-> impl Shape{
    // let sq:Squre = Squre{
    //     len:15,
    //     color:String::from("Red"),
    //     line_wid:10,
    // };
    // sq

    let re:Rectangle = Rectangle{
        len:10,
        wid:20,
        color:String::from("Black"),
        line_wid:5,
    };
    re

    // let x = false;
    // if x{
    //     sq
    // }
    // else{
    //     re
    // }
}

struct Circle{
    radius:f32,
}

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

impl Draw for Squre{
    fn draw_shape(&self){
        println!("Drawing the Squre");
    }
}
impl Draw for Rectangle{
    fn draw_shape(&self){
        println!("Drawing the Rectangle");
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

    // println!("{}",s1.area());
    // println!("{}",r1.area());

    shape_prop(s1);
    shape_prop(r1);

    let c1:Circle = Circle{
        radius:10.5,
    };
    //shape_prop(c1);

    let shape = return_shape();
    shape_prop(shape);
}

