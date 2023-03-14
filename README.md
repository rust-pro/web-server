<p align="center">
<a href="/" target="_blank">
<img src="system_structure.svg" alt="System structure">
</a></p>

<p align="center">
<a>rustc 1.62.0 (fe5b13d68 2022-05-18)</a> |
<a>Microsoft C++ Build Tools</a> |
<a>Cargo 1.62.0</a>
</p>

___

## Git Repositories

- [GitHub](https://github.com/rust-pro/web-server)
- [GitLab](https://gitlab.com/rust-inc/web-server)
- [Bitbucket](https://bitbucket.org/hainghia/web-server)

### Remote repository

```shell
git remote -v

git remote add origin git@github.com:rust-pro/web-server.git
git remote add gitlab git@gitlab.com:rust-inc/web-server.git
git remote add bitbucket git@bitbucket.org:hainghia/web-server.git


git add .; git commit -asm "Initial commit";git push origin main; git push gitlab main; git push bitbucket main
```

## Prerequisites (Windows 11) - Powershell 7.3.1

Before you want to try this on your local, here are requirements

- Your computer must have at least 16 CPUs, 16 GBs RAM
- A public key in .ssh/id_rsa.pub in your home directory: Generate an SSH key pair `ED25519 (preferred)`

### Host directory windows

```shell
start C:\Windows\System32\drivers\etc
```

---

- [SSH Commandline](https://www.ssh.com/academy/ssh/keygen)
- [Install OpenSSH for Windows](https://learn.microsoft.com/en-us/windows-server/administration/openssh/openssh_install_firstuse?tabs=powershell)

- Copying the Public Key to the Server

```shell
ssh-copy-id -i ~/.ssh/id_ed25519.pub kukun@172.16.99.100
```

```shell
ssh-keygen -t ed25519 -C "hohainghia19@gmail.com"
```

OR RSA (at least 2048-bit key size)

```shell
ssh-keygen -t rsa -b 2048 -C "hohainghia19@gmail.com"
```

Output:

```shell
cd ~/.ssh
```

---

### [Install Chocolatey](https://docs.chocolatey.org/en-us/choco/setup)

```shell
Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
```

```shell
choco upgrade chocolatey
```

---

### [Install VirtualBox](https://www.virtualbox.org/wiki/Downloads)

Download packages and install binary

```
https://download.virtualbox.org/virtualbox/7.0.4/VirtualBox-7.0.4-154605-Win.exe
```

---

### [Install Vagrant](https://developer.hashicorp.com/vagrant/downloads)

Install or update to v2.3.4 (the latest version) of Vagrant to get started.

```
https://releases.hashicorp.com/vagrant/2.3.4/vagrant_2.3.4_windows_amd64.msi
```

#### [Search for boxes by operating system, included software, architecture and more](https://app.vagrantup.com/boxes/search?provider=virtualbox)

Example

```shell
start https://app.vagrantup.com/generic/boxes/ubuntu2204
```

#### [Vagrant Commands (CLI)](https://developer.hashicorp.com/vagrant/docs/cli)

---

### [Installing Ansible on Windows](https://docs.ansible.com/ansible/latest/installation_guide/installation_distros.html#installing-ansible-on-windows)

- [Ansible core version](https://pypi.org/project/ansible-core/)
- [Ansible release notes](https://docs.ansible.com/ansible/latest/reference_appendices/release_and_maintenance.html#releases-and-maintenance)

```shell
sudo apt-add-repository ppa:ansible/ansible
sudo apt-get update
sudo apt-get install python3-pip git libffi-dev libssl-dev -y
pip install --user ansible pywinrm
sudo apt install ansible
```

#### Ansible Commands

```shell
#Ping two guest VMs
ansible all --module-name ansible.builtin.ping --user kukun --inventory inventory.yaml

#Check current directory after login
ansible all -i inventory.yaml -m shell -a "pwd" -u kukun

#Execute playbook to install Docker on guest VMs
# Use the --verbose flag when running your playbook to see detailed output from successful modules as well as unsuccessful ones.
ansible-playbook --inventory inventory.yaml playbook.yaml --extra-vars "cluster_vip=172.16.0.16"

ansible-playbook -i inventory.yaml playbook.yaml -e 'ansible_cfg=config/ansible.cfg'

# Installing the Collection from Ansible Galaxy
ansible-galaxy collection install -r requirements.yaml
ansible-galaxy collection install community.docker
```

---

### [Install Rust](https://www.rust-lang.org/tools/install)

```shell
start https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe
```

You may need to install the [Visual Studio C++ Build tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
when prompted to do so.

```shell
start https://aka.ms/vs/17/release/vs_BuildTools.exe
```

#### [Rust command](https://rust-lang.github.io/rustup/index.html)

```shell
rustc --version
rustup update

rustup toolchain install nightly
rustup toolchain list

# Set toolchain global
rustup default nightly
rustup default stable

# Set toolchain nightly curent project
rustup override set nightly
rustup override set stable

```

#### [Cargo command](https://doc.rust-lang.org/cargo/commands/index.html)

```shell
cargo install cargo-watch
cargo install diesel_cli

rm -rf ~/.cargo/registry

cargo --version
cargo doc --open
cargo test
cargo build
cargo build --release
cargo build -r
cargo check
cargo bench
cargo run 
cargo run --bin users
cargo run --release
cargo watch --watch src --exec run
cargo update
cargo fetch
```

---

### [Postgresql](https://www.postgresql.org/download/)

```shell
start https://get.enterprisedb.com/postgresql/postgresql-15.2-1-windows-x64.exe
```

https://www.postgresql.org/download/
https://www.enterprisedb.com/downloads/postgres-postgresql-downloads
WARNING: Console code page (437) differs from Windows code page (1252) 8-bit characters might not work correctly. See
psql reference page "Notes for Windows users" for details.

```shell
chcp 1252
psql --username postgres --host localhost --port 5432
```

- Drop Database Error
  Database Name: database_name
  `*ERROR:  database "actix" is being accessed by other users*
  *DETAIL:  There are 15 other sessions using the database.*`

```shell
drop database database_name;
REVOKE CONNECT ON DATABASE database_name FROM public;
SELECT pg_terminate_backend(pg_stat_activity.pid) FROM pg_stat_activity WHERE pg_stat_activity.datname = 'database_name';
drop database database_name;
```

### [Apache Kafka](https://kafka.apache.org/quickstart)

```shell
start https://www.apache.org/dyn/closer.cgi?path=/kafka/3.4.0/kafka_2.13-3.4.0.tgz
```

### [Node.js®](https://nodejs.org/)

```shell
start https://nodejs.org/dist/v19.6.1/node-v19.6.1-x64.msi
node
http.STATUS_CODES
```

### [Diesel CLI](https://diesel.rs/guides/getting-started.html)

```shell
#Install the CLI tool
cargo install diesel_cli
```

If you run into an error like:

> *note: ld: library not found for -lmysqlclient* <br>
> clang: error: linker command failed with exit code 1 (use -v to see invocation)

By default diesel depends on the following client libraries:

- [libpq](https://www.postgresql.org/docs/current/libpq.html) for the PostgreSQL backend
- [libmysqlclient](https://dev.mysql.com/doc/c-api/8.0/en/c-api-implementations.html) for the Mysql backend
- [libsqlite3](https://www.sqlite.org/index.html) for the SQlite backend

```shell
cargo install diesel_cli --no-default-features --features postgres
```

- setting the `DATABASE_URL` environment variable

```shell
# DATABASE_URL: postgres://{user}:{password}@{hostname}:{port}/{database-name}
echo DATABASE_URL=postgres://postgres:password@localhost:5432/documents > .env
```

- Setup Diesel for your project

```shell 
diesel setup
```

- Generate a new migration with the given name, and the current timestamp as the version.

```shell
diesel migration generate create_posts
```

- Next, we’ll write the SQL for migrations:

```postgresql
CREATE TABLE posts
(
    id        SERIAL PRIMARY KEY,
    title     VARCHAR NOT NULL,
    body      TEXT    NOT NULL,
    published BOOLEAN NOT NULL DEFAULT FALSE
)
```

And

```postgresql
DROP TABLE posts
```

- Runs all pending migrations.

```shell
diesel migration run
```

- Reverts and re-runs the latest migration. Useful for testing that a migration can in fact be reverted.

```shell
diesel migration redo --all
```

- Reverts the specified migrations.

```shell
diesel migration revert --all
```

- Returns true if there are any pending migrations.

```shell
diesel migration pending
```

- Lists all available migrations, marking those that have been applied.

```shell
diesel migration list
```

### [CMake](https://cmake.org/download/#latest)

> **(This is needed for the rust-rdkafka crate to work).** <br>
> Add Cmake path `C:\Program Files\CMake\bin` into environment variable in Window

```shell
choco install make
make supergraph
```

```shell
start https://github.com/Kitware/CMake/releases/download/v3.25.2/cmake-3.25.2-windows-x86_64.msi
```

### [LLVM](https://releases.llvm.org/download.html)

> **(This is needed for the [argonautica](https://crates.io/crates/argonautica) crate to work).** <br>
> Add Cmake path `C:\Program Files\CMake\bin` into environment variable in Window

```shell
start https://github.com/llvm/llvm-project/releases/download/llvmorg-15.0.7/LLVM-15.0.7-win64.exe
```

## Run Apollo Router with Environment variable

https://github.com/apollographql/supergraph-demo-fed2/tree/main/supergraph

```shell
cargo run --bin apollo-router -- -s ./apollo-router/supergraph.graphql -c ./apollo-router/router.yaml
```

- Generate Secret Key 256-bit base64 key: ASN.1 DER formatted private key

```shell
choco install openssl
openssl rand -base64 32
openssl genrsa -out rocket.pem 512
openssl rsa -in rocket.pem -out rocket.der -outform der
```

___

### Go Lang migrate CLI

- Installation Windows

```shell
Set-ExecutionPolicy RemoteSigned -Scope CurrentUser
irm get.scoop.sh | iex
scoop install migrate
```

- Usage

```shell
migrate -help
migrate -source file://./src/database/migrations -database postgres://postgres:password@localhost:5432/documents?sslmode=disable up
```

# Docker

```shell
docker kill $(docker ps -aq); docker rm $(docker ps -aq); docker rmi $(docker images -aq); docker volume prune -f;

make stop-user-development; make build-user-development; make start-user-development
make stop-user-production; make build-user-production; make start-user-production
```

# Apollo Router Manager

https://www.apollographql.com/docs/federation/quickstart/local-composition/

## Install the Rover CLI

MacOS / Unix-like

```shell
curl -sSL https://rover.apollo.dev/nix/latest | sh
```

Windows

```shell
iwr 'https://rover.apollo.dev/win/latest' | iex
```

- Add path to environment variable `C:\Users\username\.rover\bin`

## The Apollo Router

https://www.apollographql.com/docs/router/quickstart/

Automatic download (Linux, OSX, WSL, Windows Git Bash) Doesn't run on Powershell

```shell
curl -sSL https://router.apollo.dev/download/nix/latest | sh
```

If running on Powershell need to download: https://github.com/apollographql/router/releases
Unzip and Copy the `router.exe` file to the project root directory

## Compose a supergraph schema based on a supergraph configuration file

`By installing this plugin, you accept the terms and conditions outlined by this license.
More information on the ELv2 license can be found here: https://go.apollo.dev/elv2.
Do you accept the terms and conditions of the ELv2 license? [y/N]
error: This command requires that you accept the terms of the ELv2 license.`

### Run:

```shell
rover supergraph compose --config ./supergraph.yaml
```

Instead of

```shell
# Creates prod-schema.graphql or overwrites if it already exists
rover supergraph compose --config ./supergraph.yaml > supergraph.graphql
```

## Provide the composed schema to the router

```shell
./router --supergraph=supergraph.graphql
```

***
Check Speed Function

```rust
let start = Instant::now();
// Code
my_function();
//
let duration = start.elapsed();
println!("Time elapsed: {:?}", duration);
```

Out put:

```
Time elapsed: 3.9478ms
```

---

## Copy public key and private key from Window to WSL

```shell
cp -r /mnt/c/Users/haing/.ssh ~/.ssh
sudo cp -r /mnt/c/Users/haing/.ssh /root/.ssh
```

---

# Deploy application system to AWS EC2 server

[user.kukun.site](http://user.kukun.site/)

```shell
#- Name: ubuntu
#- Region: Singapore
#- Platform: Ubuntu (Linux/UNIX)
#- Public IPv4 address: 18.136.104.245
#- Domain: kukun.site
ssh ubuntu@18.136.104.245
```

```shell
#- Name: Seminar Docker
#- Region: Singapore
#- Platform: Ubuntu (Linux/UNIX)
#- Public IPv4 address: 3.1.202.29
#- Domain: kukun.site
ssh ubuntu@3.1.202.29
```

```shell
#- Name: Server Gia Bao
#- Region: US
#- Platform: Ubuntu (Linux/UNIX)
#- Public IPv4 address: 3.92.132.157
#- Domain: kukun.site
ssh ubuntu@3.92.132.157
```

#### authorized_keys

```shell
Seminar Docker

ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIOKE7tRp++q/EjFH/0kr2Bysg70FFHo4tHNTZMpCHI7H private_key
ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIA1ZEgTK1fAQRL1o920apLM8nZaklDmunVlxB6WJDfzL NTA-PC
ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIMr9ZCl6TOwf7KoCUI8JQrlhRUACUDHqQ/8VNV59nlGL LapTop
```