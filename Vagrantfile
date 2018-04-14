# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure(2) do |config|
  config.vm.box = "debian/stretch64"

  config.vm.network :forwarded_port, guest: 8070, host: 8070, host_ip: "127.0.0.1"

  # install rust and gcc
  config.vm.provision "shell", inline: <<-SHELL
    apt-get update
    apt-get install -y build-essential devscripts daemon pkg-config libssl-dev

    wget --no-verbose -O rustup-init \
      https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init
    chmod a+x rustup-init
    su -c './rustup-init -vy' vagrant
    rm rustup-init
  SHELL

end
