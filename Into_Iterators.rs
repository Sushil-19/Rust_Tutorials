/*
Into Iterators
*/

// IntoIterator {
//     type Item;
//     type IntoIter;
//     fn into_iter(self) -> Self::IntoIter{
//     }
// }

struct Book{
    name:String,
    author:String,
    genere:String,
}

// struct BookIter{
//     properties: Vec<String>,
// }

// impl Iterator for BookIter{
//     type Item = String;
//     fn next(&mut self) -> Option<Self::Item>{
//         self.properties.pop()
//     }
// }

impl IntoIterator for Book{
    type Item= String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter{
        vec![self.name, self.author, self.genere].into_iter()
    }
}

fn main(){
    let b1: Book = Book{
        name:"RamLal Stories".to_string(),
        author:"RamLal".to_string(),
        genere:"Horror".to_string(),
    };

    let mut bookiter = b1.into_iter();

    println!("{:?}", bookiter.next());
    println!("{:?}", bookiter.next());
    println!("{:?}", bookiter.next());
    println!("{:?}", bookiter.next());
}


