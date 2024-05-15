/*
    conditions,
    match
*/

fn main(){
    let marks:i32 = 90;
    let mut grade:char = 'N';


    let ans:char = if marks>90 {
        'A'
    }
    else if marks>80 {
        'B'
    }
    else if marks>=70 {
        'C'
    }
    else {
        'F'
    };
    
    match marks {
        90..=100 => grade='A',
        80..=89 => grade='B',
        70..=79 => grade='C',
        _ =>grade = 'F',
    }

    println!("{}", ans);

}