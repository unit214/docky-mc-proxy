use clap::{Parser, Subcommand};
use std::fs::{OpenOptions, metadata, copy, read_to_string, remove_file, read_dir};
use std::io::{self, prelude::*};
use std::path::{Path, PathBuf};
use regex::Regex;
use std::process::{Command, ExitCode, exit};


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

}

#[derive(Subcommand, Debug)]
enum Commands {
    Add {
        /// Subdomain
        #[arg(short, long)]
        subdomain: String,

        /// Force / overwrite
        #[arg(short, long)]
        force: bool,

        /// Which port the redirect should go to
        #[arg(short, long)]
        port: u16,
    },
    Remove {
        /// Subdomain
        #[arg(short, long)]
        subdomain: String,
    },
    List {},
    Init {},
}

fn file_exists(path: &PathBuf) -> bool {
    metadata(path).is_ok()
}

const CONFIG_FOLDER: &str = "./conf/";
const TEMPLATE: &str = "./conf/base.conf.template";
const DOMAIN: &str = ".traefik.me";

fn main() -> ExitCode {
    let args = Args::parse();

    match &args.command {
        Some(Commands::Init {}) => {
            let env = std::env::vars();
            for (key, value) in env {
                if key.starts_with("D_") {
                    let domain = key.replace("D_", "");
                    let port = value.parse::<u16>().unwrap();
                    create_new_domain(&domain, &port, &true)?;
                }
            }
        },
        Some(Commands::List {}) => {
            // read dir
            if let Ok(entries) = read_dir(CONFIG_FOLDER) {
                for entry in entries {
                    let entry = entry?;
                    let path = entry.path();
                    if path.is_file() {
                        let contents = read_to_string(&path)?;
                        let domain_placeholder = extract_domain_placeholder(&contents);
                        let port_placeholder = extract_port_placeholder(&contents);
                        if let (Some(domain), Some(port)) = (domain_placeholder, port_placeholder) {
                            println!("{} --> localhost:{}", domain, port);
                        }
                    }
                }
            }
        }
        Some(Commands::Add { force, subdomain, port }) => {
           create_new_domain(&subdomain, &port, force)?;
        }
        Some(Commands::Remove { subdomain }) => {
            let full_domain = parse_subdomain(&subdomain);
            let full_path = Path::new(CONFIG_FOLDER).join(format!("{}{}", &full_domain, ".conf"));

            if !file_exists(&full_path) {
                println!("File does not exists. No changes are made.");
                // Return exit 1 err code
                return ExitCode::from(1);
            }

            remove_file(&full_path)?;
        }
        None => {
            println!("No command provided. Exiting");
            return ExitCode::from(1);
        }
    }

    // do not run if command is list
    if let Some(Commands::List {}) = &args.command {
        return ExitCode::from(0);
    }
    let output = Command::new("nginx")
        .arg("-s")
        .arg("reload")
        .output()?;

    if output.status.success()  {
        println!("Nginx restarted successfully");
    }

    ExitCode::from(0)
}

fn create_new_domain(subdomain: &str, port: &u16, force: &bool) -> io::Result<()> {
    let full_domain = parse_subdomain(&subdomain);
    let full_path = Path::new(CONFIG_FOLDER).join(format!("{}{}", &full_domain, ".conf"));

    // check if file exists
    if file_exists(&full_path) && !force {
        println!("File exists. Please use the --force or -f parameter to overwrite");
        return exit(1);
    }

    // Copy base config
    copy(TEMPLATE, &full_path)?;

    // update base config with args
    let contents = read_to_string(&full_path)?;
    let new = contents.replace("XXXX", &*port.to_string()).replace("__DOMAIN_PLACEHOLDER__", &full_domain);

    let mut file = OpenOptions::new().write(true).truncate(true).open(&full_path)?;
    file.write(new.as_bytes())?;
    Ok(())
}

fn extract_domain_placeholder(contents: &str) -> Option<String> {
    let re = Regex::new(r"server_name\s+(\S+);").unwrap();
    if let Some(captures) = re.captures(contents) {
        captures.get(1).map(|m| m.as_str().to_string())
    } else {
        None
    }
}

fn extract_port_placeholder(contents: &str) -> Option<String> {
    let re = Regex::new(r"proxy_pass http://localhost:(\d+);").unwrap();
    if let Some(captures) = re.captures(contents) {
        captures.get(1).map(|m| m.as_str().to_string())
    } else {
        None
    }
}

fn parse_subdomain(subdomain: &str) -> String {
    let result = if !subdomain.contains(DOMAIN) {
        subdomain.to_string() + DOMAIN
    } else {
        subdomain.to_string()
    };
    result.to_lowercase()
}