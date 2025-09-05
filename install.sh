echo "Name? [service]: "
read name
curl -L https://github.com/Sushi-Mampfer/nest-setup/releases/latest/download/nest-service --create-dirs -o ~/.local/bin/$name
