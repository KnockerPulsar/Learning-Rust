use std::collections::{HashMap, HashSet};

pub fn test_commands() {
    let possible_commands = vec![
        "Add x to y. Where x is a person's name and y is a department name",
        "Show department dep. Where dep is the name of an existing department",
        "Show company. Doesn't require any parameters",
    ];

    println!("Possible commands: ");
    for command in possible_commands {
        println!("{}", command);
    }
    // Company:
    //      Department:
    //          Employee
    let mut company: HashMap<String, HashSet<String>> = HashMap::new();

    let test_cases = vec![
        "Add Sara to CS",
        "add tarek to CS",
        "Add Jimmy to AI",
        "Add Kimo to farming",
        "Add Wael to DevOps",
        "Show department CS",
        "Show company",
    ];

    for (case_index, test_case) in test_cases.iter().enumerate() {
        println!("{}", test_case);
        let command_result = command(&mut company, test_case.to_string());

        match command_result {
            Ok(_) => {
                println!("Test {}/{} passed", case_index + 1, test_cases.len());
            }
            Err(_) => {
                println!("Test {}/{} failed", case_index + 1, test_cases.len());
            }
        }
    }
}

fn print_dep_sorted(company: &HashMap<String, HashSet<String>>, dep_name: &String) {
    let dep_set = company.get(dep_name).unwrap();
    let mut department_vec: Vec<&String> = dep_set.iter().collect();
    department_vec.sort();
    println!("{} : {:#?}", dep_name, department_vec);
}

fn add_emp_to_dep(
    company: &mut HashMap<String, HashSet<String>>,
    command_words: &Vec<&str>,
) -> Result<(), String> {
    let emp_name = command_words[1].to_string();
    let dep_name = command_words.last().unwrap().to_string();

    let dep = company.entry(dep_name).or_insert(HashSet::new());

    match dep.get(&emp_name) {
        Some(_) => Ok(()),
        None => {
            dep.insert(emp_name);
            println!("{:#?}", company);
            Ok(())
        }
    }
}

fn show_department(
    company: &HashMap<String, HashSet<String>>,
    command_words: &Vec<&str>,
) -> Result<(), String> {
    let dep_name = command_words[2];
    let dep = company.get(dep_name);
    match dep {
        Some(_) => {
            print_dep_sorted(&company, &String::from(dep_name));
            Ok(())
        }
        None => Err(String::from("Requested department does not exist")),
    }
}

fn show_company(company: &HashMap<String, HashSet<String>>) -> Result<(), String> {
    let mut dep_names: Vec<&String> = company.keys().collect();
    dep_names.sort();
    for dep in dep_names {
        print_dep_sorted(&company, &String::from(dep));
    }

    Ok(())
}

fn command(company: &mut HashMap<String, HashSet<String>>, command: String) -> Result<(), String> {
    let lowercase = command.to_ascii_lowercase();
    let command_words: Vec<&str> = lowercase.split_whitespace().collect();
    let base_command = command_words[0];

    match base_command {
        "add" => add_emp_to_dep(company, &command_words),

        "show" => match command_words[1] {
            "department" => show_department(&company, &command_words),
            "company" => show_company(&company),
            _ => Err(String::from("Invalid show target")),
        },
        _ => Err(String::from("Invalid command")),
    }
}
