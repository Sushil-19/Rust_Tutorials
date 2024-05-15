/*
Result Enums
*/


struct Student{
    name:String,
    grade:Option<u32>,
}

fn check_grades(stud_name:String, student_db:&Vec<Student>) -> Result<Option<u32>, String> {
    for _student @ ref sref in student_db{
        if sref.name == stud_name{
            return Ok(sref.grade);
        }
    }
    Err(String::from("The student is not in Database"))
}

fn main(){
    let student_db:Vec<Student> = vec![
        Student{
            name:String::from("RamLal"),
            grade:Some(85),
        },
        Student{
            name:String::from("ShamLal"),
            grade:Some(60),
        },
        Student{
            name:String::from("HariLal"),
            grade:None,
        },
    ];

    let stud_record:Result<Option<u32>,String> = check_grades("Ramu".to_string(), &student_db);

    match stud_record{
        Ok(Some(grade)) => println!("The grade is {}", grade),
        Ok(None) => println!("No grade found"),
        Err(err_msg)=> {println!("Error is {}",err_msg)},
    }

}


// enum Result<T, E>{
//     Ok(T),
//     Err(E),
// }

