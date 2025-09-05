# Nest Setup
A small executable to create and manage services on nest.

## Installation
### Easy
`curl -sSf https://raw.githubusercontent.com/Sushi-Mampfer/nest-setup/refs/heads/master/install.sh | sh`

### Self compiled
1. Clone the repository: `git clone https://github.com/Sushi-Mampfer/nest-setup`
2. cd into it: `cd nest-setup`
3. Build it: `cargo build --release`
4. Make the bin dir: `mkdir ~/.local/bin`
5. Move the executable to it: `cp target/release/nest-setup ~/.local/bin/service`
   (this names it service, you can change it to whatever you want)
6. Use it: `service`
   (Or whatever you named it)

## Auto port
The executable can automatically select a free port, to use it replace the port in your program with the enviroment variable `PORT`.

## Usage
Usage: service <COMMAND>

Commands:
  create   creates a service
  start    starts a service
  stop     stops a service
  restart  stops and starts a service
  enable   enables a service
  disable  disables a service
  delete   deletes a service
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
