//Created: 2023-10-03
//Purpose: Given a name of a kattis problem, this till initialize a rust project with the correct structure and files.

use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::env::args;
use std::path::Path;
use std::process::Command;

extern crate chrono;
use chrono::prelude::*;

fn todays_date() -> String {
    let now = Local::now();
    now.format("%Y-%m-%d").to_string()
}

fn alter_cargo_toml(problem_path: &str, problem_name: &str) -> std::io::Result<()> {
    let cargo_toml_path = format!("{}/Cargo.toml", problem_path);

    // Open Cargo.toml for appending
    let mut cargo_toml = OpenOptions::new()
        .append(true)
        .open(&cargo_toml_path)?;

    // Write the [[bin]] section
    write!(cargo_toml, "\n[[bin]]\nname = \"{}\"\npath = \"src/{}.rs\"\n", problem_name, problem_name)?;

    Ok(())
}

fn main() {
    let mut args: Vec<String> = args().collect();
    if args.len() != 1 && args.len() != 2 {
        println!("Usage: ks-initiator <problem-name>");
        return;
    } else if args.len() == 1 {
        // Take user input and append to args
        println!("Enter problem name:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim();
        args.push(input.to_string());
    } else {
        println!("Using problem name: {}", args[1]);
    }

    // Change directory to /problems
    let path = "./problems";
    if !Path::new(path).exists() {
        println!("Directory {} does not exist", path);
        return;
    }

    // Check if the problem already exists
    let problem_name = &args[1];
    let problem_path = format!("{}/{}", path, problem_name);
    if Path::new(&problem_path).exists() {
        println!("Problem {} already exists", problem_name);
        return;
    }

    // cd to /problems, then creating the project using cargo new <problem-name>
    std::env::set_current_dir(path).expect("Failed to change directory");
    let output = Command::new("cargo")
        .arg("new")
        .arg(problem_name)
        .output()
        .expect("Failed to create problem");

    // Check if the command was successful
    if !output.status.success() {
        println!("Failed to create problem {}", problem_name);
        return;
    }

    // cd back to root
    std::env::set_current_dir("../").expect("Failed to change directory");

    // Print full path to the problem
    println!("Creating problem {} at {}", problem_name, problem_path);

    // Removing main.rs and replacing it with a template named <problem-name>.rs
    // Path structure is: /problems/<problem-name>/src/main.rs
    // Template created by program.
    let main_path = format!("{}/src/main.rs", problem_path);
    fs::remove_file(&main_path).expect("Failed to remove main.rs");
    let template_path = format!("{}/src/{}.rs", problem_path, problem_name);

    ////////////////////////////////////////
    {// Scope for Creating the template file
    let mut template = File::create(&template_path).expect("Failed to create template file");

    // Writing the template to the file
    let todays_date = todays_date();
    let boilerplate_code = "use std::io;\n\nfn main() {\n\tprintln!(\"Hello, world!\");\n}";
    let template_content = format!(
"//Created: {} by @VictorieeMan
//https://open.kattis.com/problems/{}
//Repository URL: https://github.com/VictorieeMan/Kattis_Solutions

{}"
        , todays_date, problem_name, boilerplate_code
    );

    template.write_all(template_content.as_bytes()).expect("Failed to write to template file");
    }

    alter_cargo_toml(&problem_path, &problem_name).expect("Failed to alter Cargo.toml");

    // ///////////////////////////////////////////////
    // {//Add a .vscode folder with a launch.json file
    // let mut launch_json = File::create(format!("{}/.vscode/launch.json", problem_path)).expect("Failed to create launch.json");
    // let launch_json_content = format!(
    //     r#"{{
    //         "version": "0.2.0",
    //         "configurations": [
    //             {{
    //                 "name": "Debug Rust Executable",
    //                 "type": "cppdbg",
    //                 "request": "launch",
    //                 "program": "${{workspaceFolder}}/target/debug/${{workspaceFolderBasename}}",
    //                 "args": [],
    //                 "stopAtEntry": false,
    //                 "cwd": "${{workspaceFolder}}",
    //                 "environment": [],
    //                 "externalConsole": true,
    //                 "MIMode": "gdb",
    //                 "setupCommands": [
    //                     {{
    //                         "description": "Enable pretty-printing for gdb",
    //                         "text": "-enable-pretty-printing",
    //                         "ignoreFailures": true
    //                     }}
    //                 ],
    //                 "preLaunchTask": "cargo build",
    //                 "miDebuggerPath": "/usr/bin/gdb",
    //                 "linux": {{
    //                     "MIMode": "gdb"
    //                 }},
    //                 "windows": {{
    //                     "MIMode": "gdb",
    //                     "miDebuggerPath": "C:\\path\\to\\gdb.exe"
    //                 }},
    //                 "osx": {{
    //                     "MIMode": "lldb"
    //                 }}
    //             }}
    //         ]
    //     }}"#
    // );
    // launch_json.write_all(launch_json_content.as_bytes()).expect("Failed to write to launch.json");
    // }

    // Done!
    println!("Problem {} created successfully", problem_name);
}
