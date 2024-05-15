/*
Combinators
*/

fn main(){
    let v: Vec<&str> = vec!["RamLal", "DhaniyaLal", "Ramprasad", "ShamLal", "SharukhKhan"];
    // let mut res: Vec<String> = vec![];
    // for word in v{
    //     if word.starts_with("R") || word.starts_with("S"){
    //         let new_word = word.to_uppercase();
    //         res.push(new_word);
    //     }
    // }
    // println!("{:?}", res);

    let res:Vec<String> = v
        .into_iter()
        .filter(|&word| word.starts_with("R") || word.starts_with("S"))
        .map(|word| word.to_uppercase())
        .collect();

    println!("{:?}", res);
}


