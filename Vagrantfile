# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure(2) do |config|
  config.vm.box = "debian/bookworm64"

  config.vm.network :forwarded_port, guest: 8070, host: 8070, host_ip: "127.0.0.1"

  config.vm.provider "virtualbox" do |v|
    v.memory = 2048
  end
  config.vm.provider "libvirt" do |domain|
    domain.memory = 2048
  end

  # install rust and gcc
  config.vm.provision "shell", inline: <<-SHELL
    apt-get update
    apt-get install -y build-essential devscripts pkg-config libssl-dev rsync

    wget --no-verbose -O rustup-init \
      https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init
    chmod a+x rustup-init
    su -c './rustup-init --profile minimal -vy' vagrant
    rm rustup-init
  SHELL

end
