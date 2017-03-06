# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure(2) do |config|
  config.vm.box = "debian/jessie64"

  # install rust and gcc
  config.vm.provision "shell", inline: <<-SHELL
    apt-get update
    apt-get install -y gcc

    wget --no-verbose https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init
    chmod a+x rustup-init
    su -c './rustup-init -vy' vagrant
    rm rustup-init
  SHELL

  # Disable automatic box update checking. If you disable this, then
  # boxes will only be checked for updates when the user runs
  # `vagrant box outdated`. This is not recommended.
  # config.vm.box_check_update = false
end
