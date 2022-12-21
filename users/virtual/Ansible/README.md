![Ansible Key Concepts](logo.svg?raw=true "logo.svg")
___

# Documentation

[intro_inventory](https://docs.ansible.com/ansible/latest/inventory_guide/intro_inventory.html)

## Prerequisites

Before you want to try this on your local, here are requirements

To install Ansible on WSL, the following commands can be run in the bash terminal:

```shell
sudo apt-add-repository ppa:ansible/ansible
```

```shell
sudo apt-get update
sudo apt-get install python3-pip git libffi-dev libssl-dev -y
pip install --user ansible pywinrm
sudo apt install ansible
```

If you encounter timeout errors when running Ansible on the WSL, this may be due to an issue with sleep not returning
correctly. The following workaround may resolve the issue:

```shell
mv /usr/bin/sleep /usr/bin/sleep.orig
ln -s /bin/true /usr/bin/sleep
```

Another option is to use WSL 2 if running Windows 10 later than build 2004.

```shell
wsl --set-default-version 2
```

## Provision VMs

```shell
vagrant up
```

Let's ssh into two guest VMs

```shell
ssh ci@172.16.16.11
```

```shell
ssh ci@172.16.16.12
```

## Ad-hoc Commands

Ping two guest VMs

```shell
ansible all -i inventory.yaml -m ping -u ci
```

Check current directory after login

```shell
ansible all -i inventory.yaml -m shell -a "pwd" -u ci
```

## Playbook

See all available  [Ansible modules](https://docs.ansible.com/ansible/2.9/modules/list_of_all_modules.html)

Execute playbook to install Docker on guest VMs

```shell
ansible-playbook -i inventory.yaml playbook.yml
```

## Copy public key and private key from Window to WSL

User Root

```shell
sudo su
cd /root/.ssh
sudo cp /mnt/c/Users/haing/.ssh/id_rsa .
sudo cp /mnt/c/Users/haing/.ssh/id_rsa.pub .
sudo chmod -R 0600 id_rsa
```

User hainghia

```shell
cd /home/hainghia/.ssh
cp /mnt/c/Users/haing/.ssh/id_rsa .
cp /mnt/c/Users/haing/.ssh/id_rsa.pub .
chmod -R 0600 id_rsa
```
