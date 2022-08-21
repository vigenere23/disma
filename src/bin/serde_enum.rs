use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct A {
    pub roles: Vec<Role>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum Role {
    Manual(ManualRole),
    FromTemplate(TemplateRole),
}

#[derive(Debug, Serialize, Deserialize)]
struct ManualRole {
    pub name: String,
    pub permissions: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TemplateRole {
    pub name: String,
    pub template: String,
}

fn main() {
    let file_path = "test.json";
    // let a = A {
    //     roles: vec![
    //         Role::Manual(ManualRole {
    //             name: String::from("admin"),
    //             permissions: String::from("ADMINISTRATOR"),
    //         }),
    //         Role::FromTemplate(TemplateRole {
    //             name: String::from("team1"),
    //             template: String::from("team"),
    //         }),
    //     ],
    // };
    // let x = serde_json::to_string_pretty(&a).unwrap();
    // fs::write(file_path, x).unwrap();

    let x_back = fs::read_to_string(file_path).unwrap();
    let a_back: A = serde_json::from_str(&x_back).unwrap();

    // println!("{:#?}", a);
    println!("{:#?}", a_back);
}
