

fn main(){
    'outer: loop{
        println!("This is a outer loop");
        loop {
            println!("This is an inner loop");
            break 'outer;
        }
    }
    let vec:Vec<i32> = vec![10,20,30,40,50];

    for j in vec{
        println!("{j}");
    }

    let mut val:i32 = 0;
    while val<10{
        val=val+1;
    }
    println!("Val is : {}",val);
}