/*
Structs and Option enum.
*/

struct Student{
    name:String,
    grade:Option<u32>,
}

fn get_grades(stud_name:String, student_db:&Vec<Student>) -> Option<u32> {
    for _student @ ref sref in student_db{
        if sref.name == stud_name{
            return sref.grade;
        }
    }
    None
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

    let grade = get_grades("ShamLal".to_string(), &student_db);

    match grade{
        Some(grade) => println!("The grade is {}", grade),
        None=> {},
    }

}

// enum Option<T>{
//     None,
//     Some(T),
// }

