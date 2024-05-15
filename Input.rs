// macro_rules! input{
//     ($t: ty) => {{
//         let mut n = String::new();
//         std::io::stdin()
//         .read_line(&mut n)
//         .expect("Failed to read input");

//         let n:$t = n.trim().parse().expect("Invalid input");
//         n
//         }
//     };
// }

macro_rules! add_as{
    ($a:expr, $b:expr, $c:ty)=>{
        $a as $c + $b as $c
    };
}

macro_rules! ident_macro{
    ($x:ident) => {
        //$x = $x+1;
        println!("Value of x is : {}", $x);
    };
}

macro_rules! fn_macro{
    ($nm:ident, $a:ident, $b:ty) => {
    
        fn $nm ($a:$b){
            println!("The value is {:?}, tpe is {:?}", $a, stringify!($b));
        }
        
    }
}

fn_macro!(f1, val, i32);

fn main(){
    // let something = input!(String);
    // println!("{}", something);
    println!("The res is {}", add_as!(1,2.5,f32));
    let mut x:i32 = 10;
    ident_macro!(x);
    
    f1(20);
}