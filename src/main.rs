// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

fn main() {
    let mut company: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();

    loop {
        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let input: Vec<&str> = input.trim().split_whitespace().collect();

        if input.len() == 4 {
            todo!("TODO: Add employee to department")
        } else if input.len() == 3 {
            todo!("TODO: List employee")
        } else {
            println!("Unknown command {:?}", input.concat())
        }
    }
}
