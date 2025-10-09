#!/bin/sh

printf "Name for the executable? [service]: "
read name < /dev/tty

if [ -z "$name" ]; then
  name="service"
fi

curl -sSfL https://github.com/Sushi-Mampfer/nest-setup/releases/latest/download/nest-setup --create-dirs -o "$HOME/.local/bin/$name"

chmod +x "$HOME/.local/bin/$name"
