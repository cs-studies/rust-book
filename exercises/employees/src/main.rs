// Using a hash map and vectors, create a text interface to allow a user to add
// employee names to a department in a company. For example, “Add Sally to
// Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all
// people in a department or all people in the company by department, sorted
// alphabetically.

use std::collections::HashMap;
use std::io;

fn main() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let employee_name = get_input("Enter employee name or 1 to exit:");
        if employee_name == "1" {
            break;
        }
        let department_name = get_input("Enter department name:");

        let department_employees = departments.entry(department_name).or_default();
        department_employees.push(employee_name);
    }

    let answer = get_input("List all people in a department? [y/n]");
    if answer == "y" {
        let department = get_input("Enter department name:");
        if departments.contains_key(&department) {
            let employees = departments.get(&department).unwrap();
            println!("Employees:");
            for e in employees {
                println!("{:?}", e);
            }
        } else {
            println!("Department {} not found.", department);
        }
    }

    let answer = get_input("List all employees, sorted? [y/n]");
    if answer == "y" {
        let mut departments_sorted: Vec<_> = departments.iter().collect();
        departments_sorted.sort_by_key(|t| t.0);
        for (d, employees) in departments_sorted {
            println!("Department {}:", d);
            let mut employees_sorted = employees.clone();
            employees_sorted.sort();
            for e in employees_sorted {
                println!("{}", e);
            }
        }
    }
}

fn get_input(message: &str) -> String {
    println!("{}", message);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().to_string(),
        Err(err) => panic!("Error: {}", err),
    }
}
