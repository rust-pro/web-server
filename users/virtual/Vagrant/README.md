# [VirtualBox](https://www.virtualbox.org/wiki/Downloads)

# [Vagrant](https://www.vagrantup.com/)

[Vagrant downloads](https://developer.hashicorp.com/vagrant/downloads)

[Vagrant Cloud App](https://app.vagrantup.com/boxes/search)

## [Vagrant Commands (CLI)](https://developer.hashicorp.com/vagrant/docs/cli)

Autocompletion

```shell
vagrant autocomplete install --bash --zsh
```

This initializes the current directory to be a Vagrant environment by creating an initial Vagrantfile if one does not already exist.

```shell
vagrant init
vagrant init --minimal --force
```

Creates and configures guest machines according to your Vagrantfile.

```shell
vagrant up
```

SSH into a running Vagrant machine and give you access to a shell.

```shell
vagrant ssh
```

Shuts down the running machine Vagrant is managing.

```shell
vagrant halt
```

The equivalent of running a halt followed by an up.

```shell
vagrant reload
```

This command stops the running machine Vagrant is managing and destroys all resources that were created during the machine creation process.

```shell
vagrant destroy
```

# Generate an SSH key pair

```shell
ssh-keygen -t rsa -b 2048 -C "hohainghia19@gmail.com"
```

```shell
start ~/.ssh/id_rsa.pub
```

# Permission denied

`vagrant@127.0.0.1: Permission denied (publickey,gssapi-keyex,gssapi-with-mic).`

```shell
$Env:VAGRANT_PREFER_SYSTEM_BIN += 0
```

# Install Plugins

```shell
vagrant plugin install vagrant-vbguest
```
