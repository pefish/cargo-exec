use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::env;
use std::process::Command;
use std::fmt::Debug;

#[derive(Deserialize, Debug)]
struct Config {
    package: Package
}

#[derive(Deserialize, Debug)]
struct Package {
    metadata: Metadata
}

#[derive(Deserialize, Debug)]
struct Metadata {
    scripts: HashMap<String, String>
}

fn main() {
    let mut f = File::open("Cargo.toml").expect("Cargo.toml file not found.");

    let mut toml = String::new();
    f.read_to_string(&mut toml)
        .expect("Failed to read Cargo.toml.");

    let table : Config = toml::from_str(&toml)
        .expect("Expected Cargo.toml to contain package.metadata.scripts table.");

    let args = parse(env::args().collect());

    match args.script {
        None => {
            // display the name of all scripts
            table.package.metadata.scripts.keys()
                .for_each(|script_name| println!("{}", script_name));
        },
        Some(script_name) => {
            let script = table.package.metadata.scripts.get(&script_name)
                .expect("Script not found");
            run_script(script);
        },
    }
}

fn run_script(script: &str) {
    let mut shell = if cfg!(target_os = "windows") {
        let mut shell = Command::new("cmd");
        shell.arg("/C");

        shell
    } else {
        let mut shell = Command::new("sh");
        shell.arg("-c");

        shell
    };

    let mut child = shell.arg(script).spawn().expect("Failed to run script");

    match child.wait() {
        Ok(status) => println!("Finished, status of {}", status),
        Err(e)     => println!("Failed, error: {}", e)
    }
}


pub struct Args {
    /// name of the script to be executed as specified in Cargo.toml
    pub script: Option<String>,
}

pub fn parse<T>(mut args: Vec<T>) -> Args
    where
        T: Into<String> + Debug,
        Option<T>: PartialEq
{
    match args.pop() {
        Some(script) => {
            let script : String = script.into();
            if script.contains("exec") {
                return Args { script: None};
            };
            return Args {script: Some(script)};
        },
        None => return Args {script: None} 
    };
}
