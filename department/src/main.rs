// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company.
// For example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
use std::collections::HashMap;
use std::io;

fn add_person(input: String, departments: &mut HashMap<String, Vec<String>>) {
    let words: Vec<&str> = input.trim().split_whitespace().collect();
    let person = words.get(1);
    let department = words.get(3);

    match department {
        Some(department) => match person {
            Some(person) => {
                let dep = departments.entry(department.to_string()).or_insert(vec![]);
                dep.push(person.to_string());
                dep.sort();
            }
            None => println!("Person's name is not provided"),
        },
        None => println!("Department name is not provided"),
    }
}

fn print_all_employees(departments: &mut HashMap<String, Vec<String>>) {
    let mut people: Vec<String> = Vec::new();

    for (_, department) in departments {
        for person in department.iter() {
            people.push(person.to_string());
        }
    }

    people.sort();

    for (i, person) in people.iter().enumerate() {
        println!("{}. {}", i + 1, person);
    }
}

fn print_employees_by_department(input: String, departments: &HashMap<String, Vec<String>>) {
    let words: Vec<&str> = input.trim().split_whitespace().collect();
    let department_name = words.get(1);

    match department_name {
        Some(department_name) => {
            match departments.get(&department_name.to_string()) {
                Some(people) => {
                    for (i, person) in people.iter().enumerate() {
                        println!("{}. {}", i + 1, person);
                    }
                }
                None => println!("Unknown department name: {}", department_name),
            };
        }
        None => println!("Department name is not provided"),
    };
}

fn main() {
    println!("Type the command");

    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.to_lowercase().contains("list all") {
            print_all_employees(&mut departments)
        } else if input.to_lowercase().contains("list") {
            print_employees_by_department(input, &mut departments);
        } else if input.to_lowercase().contains("add") {
            add_person(input, &mut departments);
        } else {
            println!("Uknown input: {}", input)
        }
    }
}
