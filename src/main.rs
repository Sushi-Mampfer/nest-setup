use std::{
    env, fmt,
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
    process::Command,
};

use clap::{Parser, Subcommand};
use regex::Regex;

struct Service {
    name: String,
    description: String,
    dir: String,
    port: Option<String>,
    pre_start_cmd: Option<String>,
    start_cmd: String,
}

impl fmt::Display for Service {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[Unit]")?;
        writeln!(f, "Description={}", self.description)?;
        writeln!(f)?;
        writeln!(f, "[Service]")?;
        writeln!(f, "WorkingDirectory={}", self.dir)?;
        if let Some(port) = &self.port {
            writeln!(f, "Environment=\"PORT={}\"", port)?;
        }
        if let Some(cmd) = &self.pre_start_cmd {
            writeln!(f, "ExecStartPre=-{}", cmd)?;
        }
        writeln!(f, "[Service]")?;
        writeln!(f, "ExecStart={}", self.start_cmd)?;
        writeln!(f, "Restart=on-failure")?;
        writeln!(f)?;
        writeln!(f, "[Install]")?;
        writeln!(f, "WantedBy=default.target")
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// creates a service
    Create {},
    /// starts a service
    Start { name: String },
    /// stops a service
    Stop { name: String },
    /// enables a service
    Enable {
        name: String,

        /// also starts the service
        #[arg(short, long)]
        now: bool,
    },
    /// disables a service
    Disable {
        name: String,

        /// also stops the service
        #[arg(short, long)]
        now: bool,
    },
    /// deletes a service
    Delete {
        name: String,

        #[arg(short, long)]
        force: bool,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Create {} => {
            let home = env::var("HOME").expect("No $HOME found.");

            let name = loop {
                let name = ask("Name".to_string());
                if name != "".to_string() {
                    break name;
                }
            };

            let mut description = ask("Description [name]".to_string());
            if description == "" {
                description = name.clone();
            }

            let mut dir = ask("Directory [%h = home directory]".to_string());
            if dir == "" {
                dir = "%h".to_string();
            }

            let mut port = Some(ask("Port [auto, \"-\" = none]".to_string()));
            if port == Some("".to_string()) {
                let mut cmd = Command::new("nest");
                cmd.arg("get_port");
                let output = cmd.output().unwrap();
                let stdout = String::from_utf8_lossy(&output.stdout);
                let re = Regex::new(r"Port (\d+)").unwrap();
                if let Some(caps) = re.captures(&stdout) {
                    port = Some(caps[1].parse().unwrap());
                } else {
                    eprintln!("Unable to find port");
                    return;
                }
            } else if port == Some("-".to_string()) {
                port = None;
            }

            let mut pre_start_cmd = Some(ask("Pre start command [none]".to_string()));
            if pre_start_cmd == Some("".to_string()) {
                pre_start_cmd = None;
            }

            let start_cmd = loop {
                let start_cmd = ask("Start command".to_string());
                if start_cmd != "".to_string() {
                    break start_cmd;
                }
            };

            let service = Service {
                name: name.clone(),
                description,
                dir,
                port,
                pre_start_cmd,
                start_cmd,
            };
            let mut path = PathBuf::from(home);
            path.push(".config");
            path.push("systemd");
            path.push("user");
            path.push(format!("{}.service", service.name));
            let mut file = File::create(path).unwrap();
            write!(file, "{}", service).unwrap();

            let mut cmd = Command::new("systemctl");
            cmd.args(["--user", "deamon-reload"]);
            let status = cmd.status().unwrap();
            if !status.success() {
                eprintln!("Failed to reload deamon: {}", status);
            }

            if ask_yes("Service created, should it be enabled and started now?".to_string()) {
                let mut cmd = Command::new("systemctl");
                cmd.args(["--user", "enable", &name, "--now"]);
                let status = cmd.status().unwrap();
                if !status.success() {
                    eprintln!("Failed to enable service: {}", status);
                }
            }
        }
        Commands::Start { name } => {
            let status = Command::new("systemctl")
                .args(["--user", "start", &name])
                .status()
                .unwrap();
            if !status.success() {
                eprintln!("Failed to start service: {}", status);
            }
        }
        Commands::Stop { name } => {
            let status = Command::new("systemctl")
                .args(["--user", "stop", &name])
                .status()
                .unwrap();
            if !status.success() {
                eprintln!("Failed to stop service: {}", status);
            }
        }
        Commands::Enable { name, now } => {
            let mut cmd = Command::new("systemctl");
            cmd.args(["--user", "enable", &name]);
            if now {
                cmd.arg("--now");
            }
            let status = cmd.status().unwrap();
            if !status.success() {
                eprintln!("Failed to enable service: {}", status);
            }
        }
        Commands::Disable { name, now } => {
            let mut cmd = Command::new("systemctl");
            cmd.args(["--user", "enable", &name]);
            if now {
                cmd.arg("--now");
            }
            let status = cmd.status().unwrap();
            if !status.success() {
                eprintln!("Failed to disable service: {}", status);
            }
        }
        Commands::Delete { name, force } => {
            let home = env::var("HOME").expect("No $HOME found.");

            if !force {
                if !ask_no(format!(
                    "Do you really want to delete the service \"{}\"?",
                    name
                )) {
                    return;
                }
            }

            let status = Command::new("systemctl")
                .args(["--user", "disable", &name, "--now"])
                .status()
                .unwrap();
            if !status.success() {
                eprintln!("Failed to stop service: {}", status);
                return;
            }

            let mut path = PathBuf::from(home);
            path.push(".config");
            path.push("systemd");
            path.push("user");
            path.push(format!("{}.service", name));
            fs::remove_file(path).unwrap()
        }
    }
}

fn ask(question: String) -> String {
    print!("{}: ", question);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn ask_yes(question: String) -> bool {
    let mut input = String::new();
    print!("{} [Y/n]: ", question);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let trimmed = input.trim().to_lowercase();
    if trimmed == "n" {
        return false;
    }
    true
}

fn ask_no(question: String) -> bool {
    let mut input = String::new();

    print!("{} [y/N]: ", question);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let trimmed = input.trim().to_lowercase();
    if trimmed == "y" {
        return true;
    }
    false
}
