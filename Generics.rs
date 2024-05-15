/*
Generics
*/

struct Just<T,U>{
    x:T,
    y:U,
}

impl <T,U> Just<T,U>{
    fn new(x:T, y:U) -> Just<T,U>{
        Just{x,y}
    }
}

impl Just<i32,i32>{
    fn JustPrinting(&self){
        println!("{} {}", self.x, self.y);
    }
    fn new_1(x:i32, y:i32) -> Just<i32,i32>{
        Just{x,y}
    }
}

fn timepass<T,U>(x:Just<T,U>, y:Just<T,U>){
    unimplemented!();
}

impl Just<f32,f32>{
    fn JustPrinting(&self){
        println!("{} {}", self.x, self.y);
    }
}


fn main(){
    let j1:Just<i32,i32> = Just::new_1(10, 20);
    let j2:Just<f32,f32> = Just::new(10.5, 20.5);
    let j3:Just<f32,i32> = Just::new(10.5, 20);

    j1.JustPrinting();
    j2.JustPrinting();
}

