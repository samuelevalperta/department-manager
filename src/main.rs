// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

fn main() {
    let mut company: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();

    loop {
        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input: Vec<&str> = input.split_whitespace().collect();
        if input[0].eq_ignore_ascii_case("add") {
            let name = input[1].to_string();
            let department = input[3].to_string();
            let department = company.entry(department).or_default();
            department.push(name.to_string());
        } else if input[0].eq_ignore_ascii_case("list") {
            let department = input[1].to_string();
            if department.eq_ignore_ascii_case("all") {
                let mut all: Vec<String> = Vec::new();
                for employees in company.values() {
                    for employee in employees {
                        all.push(employee.to_string())
                    }
                }
                all.sort();
                println!("{:?}", all)
            } else {
                let department = company.get(&department);
                match department {
                    Some(employees) => {
                        let mut v = employees.clone();
                        v.sort();
                        println!("{:?}", v)
                    }
                    None => println!("Department not found"),
                }
            }
        } else {
            println!("Unknown command {:?}", input.join(" "))
        }
    }
}
