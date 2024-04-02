use reqwest::blocking::Client;
use serde_json::{from_value, to_value, Value};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};

use crate::types::GroupData;

mod types;

fn main() {
    let client = Client::new();

    let response = client
        .get("https://raw.githubusercontent.com/infinity-MSFS/groups/main/groups.json")
        .send()
        .expect("Failed to send request");

    if response.status().is_success() {
        let json_data: Value = response.json().expect("Failed to parse JSON");

        let mut data: HashMap<String, GroupData> =
            from_value(json_data).expect("Failed to deserialize JSON");

        println!("Select a group to edit:");
        for (index, (group_name, _)) in data.iter().enumerate() {
            println!("({}) {}", index + 1, group_name);
        }

        let mut input = String::new();
        print!("Enter the number of the group to edit: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let choice: usize = input.trim().parse().expect("Invalid input");

        if choice > 0 && choice <= data.len() {
            let selected_group_name = data.keys().nth(choice - 1).unwrap().clone();
            let selected_group = data.get_mut(&selected_group_name).unwrap();

            println!("Select a field to edit:");
            println!("(1) Name");
            println!("(2) Projects");
            println!("(3) Beta");
            println!("(4) Logo");
            println!("(5) Update");
            println!("(6) Path");
            println!("(7) Palette");
            print!("Enter the number of the field to edit: ");
            io::stdout().flush().unwrap();
            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");

            let field_choice: usize = input.trim().parse().expect("Invalid input");

            match field_choice {
                1 => {
                    println!("Current Name: {}", selected_group.name);
                    print!("Enter new name: ");
                    io::stdout().flush().unwrap();
                    input.clear();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read input");
                    selected_group.name = input.trim().to_string();
                    println!("Name updated successfully.");
                }
                2 => {
                    println!("Select a project to edit:");
                    for (index, project) in selected_group.projects.iter().enumerate() {
                        println!("({}) {}", index + 1, project.name);
                    }
                    print!("Enter the number of the project to edit: ");
                    io::stdout().flush().unwrap();
                    input.clear();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read input");

                    let project_choice: usize = input.trim().parse().expect("Invalid input");

                    if project_choice > 0 && project_choice <= selected_group.projects.len() {
                        let selected_project = &mut selected_group.projects[project_choice - 1];

                        println!("Select a field to edit:");
                        println!("(1) Name");
                        println!("(2) Version");
                        println!("(3) Date");
                        println!("(4) Changelog");
                        println!("(5) Overview");
                        println!("(6) Description");
                        println!("(7) Background");
                        println!("(8) Page Background");
                        println!("(9) Variants");
                        println!("(10) Package");

                        print!("Enter the number of the field to edit: ");
                        io::stdout().flush().unwrap();
                        input.clear();
                        io::stdin()
                            .read_line(&mut input)
                            .expect("Failed to read input");

                        let field_choice: usize = input.trim().parse().expect("Invalid input");

                        match field_choice {
                            1 => {
                                println!("Current Name: {}", selected_project.name);
                                println!("Enter new name: ");
                                io::stdout().flush().unwrap();
                                input.clear();
                                io::stdin()
                                    .read_line(&mut input)
                                    .expect("Failed to read input");
                                selected_project.name = input.trim().to_string();
                                println!("Name updated successfully.");
                            }
                            2 => {
                                println!("Current Version: {}", selected_project.version);
                                print!("Enter new version: ");
                                io::stdout().flush().unwrap();
                                input.clear();
                                io::stdin()
                                    .read_line(&mut input)
                                    .expect("Failed to read input");
                                selected_project.version = input.trim().to_string();
                                println!("Version updated successfully.");
                            }
                            3 => {
                                println!("Current Date: {}", selected_project.date);
                                print!("Enter new date: ");
                                io::stdout().flush().unwrap();
                                input.clear();
                                io::stdin()
                                    .read_line(&mut input)
                                    .expect("Failed to read input");
                                selected_project.date = input.trim().to_string();
                                println!("Date updated successfully.");
                            }
                            4 => {
                                println!("Current Changelog: {}", selected_project.changelog);
                                print!("Enter new changelog: ");
                                io::stdout().flush().unwrap();
                                input.clear();
                                io::stdin()
                                    .read_line(&mut input)
                                    .expect("Failed to read input");
                                selected_project.changelog = input.trim().to_string();
                                println!("Changelog updated successfully.");
                            }
                            5 => {
                                println!("Current Overview: {}", selected_project.overview);
                                print!("Enter new overview: ");
                                io::stdout().flush().unwrap();
                                input.clear();
                                io::stdin()
                                    .read_line(&mut input)
                                    .expect("Failed to read input");
                                selected_project.overview = input.trim().to_string();
                                println!("Overview updated successfully.");
                            }
                            6 => {
                                println!("Current Description: {}", selected_project.description);
                                print!("Enter new description: ");
                                io::stdout().flush().unwrap();
                                input.clear();
                                io::stdin()
                                    .read_line(&mut input)
                                    .expect("Failed to read input");
                                selected_project.description = input.trim().to_string();
                                println!("Description updated successfully.");
                            }
                            7 => {
                                println!("Current Background: {}", selected_project.background);
                                print!("Enter new background: ");
                                io::stdout().flush().unwrap();
                                input.clear();
                                io::stdin()
                                    .read_line(&mut input)
                                    .expect("Failed to read input");
                                selected_project.background = input.trim().to_string();
                                println!("Background updated successfully.");
                            }
                            8 => {
                                match &selected_project.pageBackground {
                                    Some(value) => println!("Current Page Background: {}", value),
                                    None => println!("Current Page Background: None"),
                                }
                                print!("Enter new page background (leave empty for None): ");
                                io::stdout().flush().unwrap();
                                input.clear();
                                io::stdin()
                                    .read_line(&mut input)
                                    .expect("Failed to read input");
                                let new_page_background = input.trim().to_string();
                                selected_project.pageBackground = if new_page_background.is_empty()
                                {
                                    None
                                } else {
                                    Some(new_page_background)
                                };
                                println!("Page Background updated successfully.");
                            }
                            9 => {
                                match &selected_project.variants {
                                    Some(variants) => {
                                        println!("Current Variants:");
                                        for (index, variant) in variants.iter().enumerate() {
                                            println!("({}) {}", index + 1, variant);
                                        }
                                    }
                                    None => println!("Current Variants: None"),
                                }
                                print!("Enter new variant (leave empty to keep it None): ");
                                io::stdout().flush().unwrap();
                                input.clear();
                                io::stdin()
                                    .read_line(&mut input)
                                    .expect("Failed to read input");
                                let new_variant = input.trim().to_string();
                                selected_project.variants = if new_variant.is_empty() {
                                    None
                                } else {
                                    Some(vec![new_variant])
                                };
                                println!("Variant updated successfully.");
                            }
                            10 => match &mut selected_project.package {
                                Some(package) => {
                                    println!("Current Package:");
                                    println!("(1) Owner: {}", package.owner);
                                    println!("(2) Repo Name: {}", package.repoName);
                                    println!("(3) Version: {}", package.version);
                                    println!("(4) File Name: {}", package.fileName);
                                    print!("Enter the number of the field to edit: ");
                                    io::stdout().flush().unwrap();
                                    input.clear();
                                    io::stdin()
                                        .read_line(&mut input)
                                        .expect("Failed to read input");

                                    let field_choice: usize =
                                        input.trim().parse().expect("Invalid input");

                                    match field_choice {
                                        1 => {
                                            println!("Current Owner: {}", package.owner);
                                            print!("Enter new owner: ");
                                            io::stdout().flush().unwrap();
                                            input.clear();
                                            io::stdin()
                                                .read_line(&mut input)
                                                .expect("Failed to read input");
                                            package.owner = input.trim().to_string();
                                            println!("Owner updated successfully.");
                                        }
                                        2 => {
                                            println!("Current Repo Name: {}", package.repoName);
                                            print!("Enter new repo name: ");
                                            io::stdout().flush().unwrap();
                                            input.clear();
                                            io::stdin()
                                                .read_line(&mut input)
                                                .expect("Failed to read input");
                                            package.repoName = input.trim().to_string();
                                            println!("Repo Name updated successfully.");
                                        }
                                        3 => {
                                            println!("Current Version: {}", package.version);
                                            print!("Enter new version: ");
                                            io::stdout().flush().unwrap();
                                            input.clear();
                                            io::stdin()
                                                .read_line(&mut input)
                                                .expect("Failed to read input");
                                            package.version = input.trim().to_string();
                                            println!("Version updated successfully.");
                                        }
                                        4 => {
                                            println!("Current File Name: {}", package.fileName);
                                            print!("Enter new file name: ");
                                            io::stdout().flush().unwrap();
                                            input.clear();
                                            io::stdin()
                                                .read_line(&mut input)
                                                .expect("Failed to read input");
                                            package.fileName = input.trim().to_string();
                                            println!("File Name updated successfully.");
                                        }
                                        _ => println!("Invalid field choice."),
                                    }
                                }
                                None => println!("Current Package: None"),
                            },
                            _ => {
                                println!("Invalid field choice.");
                            }
                        }
                    } else {
                        println!("Invalid project choice.");
                    }
                }
                3 => {
                    println!(
                        "Current Beta Project Background: {}",
                        selected_group.beta.background
                    );
                    print!("Enter new background for Beta Project: ");
                    io::stdout().flush().unwrap();
                    input.clear();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read input");
                    let new_background = input.trim().to_string();
                    selected_group.beta.background = new_background;

                    println!("Beta Project updated successfully.");
                }
                4 => {
                    println!("Current Logo: {}", selected_group.logo);
                    print!("Enter new logo URL: ");
                    io::stdout().flush().unwrap();
                    input.clear();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read input");
                    selected_group.logo = input.trim().to_string();
                    println!("Logo updated successfully.");
                }
                5 => {
                    println!("Current Update status: {:?}", selected_group.update);
                    print!("Enter new update status (true/false): ");
                    io::stdout().flush().unwrap();
                    input.clear();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read input");
                    let new_update_status = input.trim().parse().expect("Invalid input");
                    selected_group.update = Some(new_update_status);
                    println!("Update status updated successfully.");
                }
                6 => {
                    println!("Current Path: {}", selected_group.path);
                    print!("Enter new path: ");
                    io::stdout().flush().unwrap();
                    input.clear();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read input");
                    selected_group.path = input.trim().to_string();
                    println!("Path updated successfully.");
                }
                7 => {
                    println!("Current Primary Color: {}", selected_group.palette.primary);
                    println!(
                        "Current Secondary Color: {}",
                        selected_group.palette.secondary
                    );

                    print!("Enter new primary color: ");
                    io::stdout().flush().unwrap();
                    input.clear();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read input");
                    let new_primary_color = input.trim().to_string();

                    print!("Enter new secondary color: ");
                    io::stdout().flush().unwrap();
                    input.clear();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read input");
                    let new_secondary_color = input.trim().to_string();

                    selected_group.palette.primary = new_primary_color;
                    selected_group.palette.secondary = new_secondary_color;

                    println!("Palette updated successfully.");
                }
                _ => {
                    println!("Invalid field choice.");
                }
            }

            let updated_json =
                to_value(&data).expect("Failed to serialize data back to json, its really fucked");

            let mut file = File::create("updated_groups.json").expect("Failed to create file");
            file.write_all(updated_json.to_string().as_bytes())
                .expect("Failed to write to file");

            println!("Data updated and written to updated_groups.json successfully. Push to github manually for now too lazy to add that atm");
        } else {
            println!("Invalid group choice. Please select a valid group number.");
        }
    } else {
        println!("Failed to fetch JSON data: {}", response.status());
    }
}
