use std::fs::{copy, metadata, read_dir, read_to_string, remove_file, OpenOptions};
use std::io::{self, prelude::*};
use std::path::{Path, PathBuf};
use std::process::{exit, Command};

use clap::{Parser, Subcommand};
use regex::Regex;

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

const CONFIG_FOLDER: &str = "./conf/";
const TEMPLATE: &str = "./conf/base.conf.template";
const DOMAIN: &str = ".traefik.me";

fn main() -> io::Result<()> {
    let args = Args::parse();

    match &args.command {
        Some(Commands::Init {}) => initialize_domains()?,
        Some(Commands::List {}) => list_domains()?,
        Some(Commands::Add {
            subdomain,
            force,
            port,
        }) => create_new_domain(subdomain, *port, *force)?,
        Some(Commands::Remove { subdomain }) => remove_domain(subdomain)?,
        None => {
            println!("No command provided. Exiting");
            exit(1);
        }
    }

    // Do not reload nginx if the command is List
    if !matches!(args.command, Some(Commands::List {})) {
        reload_nginx()?;
    }

    Ok(())
}

fn file_exists(path: &PathBuf) -> bool {
    metadata(path).is_ok()
}

fn initialize_domains() -> io::Result<()> {
    let env_vars = std::env::vars();
    for (key, value) in env_vars {
        if key.starts_with("D_") {
            let domain = key.trim_start_matches("D_").to_string();
            let port = value.parse::<u16>().expect("Invalid port number");
            create_new_domain(&domain, port, true)?;
        }
    }
    Ok(())
}

fn list_domains() -> io::Result<()> {
    if let Ok(entries) = read_dir(CONFIG_FOLDER) {
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let contents = read_to_string(&path)?;
                if let (Some(domain), Some(port)) = (
                    extract_domain_placeholder(&contents),
                    extract_port_placeholder(&contents),
                ) {
                    println!("{} --> localhost:{}", domain, port);
                }
            }
        }
    }
    Ok(())
}

fn create_new_domain(subdomain: &str, port: u16, force: bool) -> io::Result<()> {
    let full_domain = parse_subdomain(&subdomain);
    let full_path = Path::new(CONFIG_FOLDER).join(format!("{}.conf", full_domain));

    if file_exists(&full_path) && !force {
        println!("File exists. Please use the --force or -f parameter to overwrite.");
        exit(1);
    }

    copy(TEMPLATE, &full_path)?;

    let contents = read_to_string(&full_path)?;
    let new_contents = contents
        .replace("XXXX", &port.to_string())
        .replace("__DOMAIN_PLACEHOLDER__", &full_domain);

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&full_path)?;
    file.write_all(new_contents.as_bytes())?;
    Ok(())
}

fn remove_domain(subdomain: &str) -> io::Result<()> {
    let full_domain = parse_subdomain(&subdomain);
    let full_path = Path::new(CONFIG_FOLDER).join(format!("{}.conf", full_domain));

    if !file_exists(&full_path) {
        println!("File does not exist. No changes made.");
        exit(1);
    }

    remove_file(full_path)?;
    Ok(())
}

fn reload_nginx() -> io::Result<()> {
    let output = Command::new("nginx").arg("-s").arg("reload").output()?;

    if output.status.success() {
        println!("Nginx restarted successfully.");
    } else {
        eprintln!("Failed to restart Nginx.");
    }

    Ok(())
}

fn extract_domain_placeholder(contents: &str) -> Option<String> {
    let re = Regex::new(r"server_name\s+(\S+);").unwrap();
    re.captures(contents)
        .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()))
}

fn extract_port_placeholder(contents: &str) -> Option<String> {
    let re = Regex::new(r"proxy_pass http://localhost:(\d+);").unwrap();
    re.captures(contents)
        .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()))
}

fn parse_subdomain(subdomain: &str) -> String {
    let result = format!("{}{}", subdomain.trim_end_matches(DOMAIN), DOMAIN);
    result.to_lowercase()
}
