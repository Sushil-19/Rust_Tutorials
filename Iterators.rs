/*
Iterators
*/

// Iterator{
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>
// }


struct Employee{
    name:String,
    age:u32,
}

struct EmployeeRecords{
    emp_db:Vec<Employee>,
}

impl Iterator for EmployeeRecords{
    type Item = String;
    fn next(&mut self) -> Option<Self::Item>{
        if self.emp_db.len() != 0{
            let res:String = self.emp_db[0].name.clone();
            self.emp_db.remove(0);
            Some(res)
        }
        else{
            None
        }
    }
}

fn main(){
    let e1: Employee = Employee{
        name:String::from("RamLal"),
        age:30,
    };
    let e2: Employee = Employee{
        name:String::from("ShamLal"),
        age:10,
    };

    let mut emp_db:EmployeeRecords = EmployeeRecords{
        emp_db:vec![e1,e2],
    };

    // println!("{:?}", emp_db.next());
    // println!("{:?}", emp_db.next());
    // println!("{:?}", emp_db.next());

    for employee in emp_db{
        println!("{}", employee);
    }
}