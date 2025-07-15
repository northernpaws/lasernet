# distro

This directory contains the PTXDist project for building the base OS images for the game supervisor computer. 

The project in this directory is based off [Pengutronix's DistroKit](https://git.pengutronix.de/cgit/DistroKit/).

## Setup

> PTXDist does not work nicely on MacOS due to MacOS's bundled bash version being <4.2. PTXDist relies in several places on case fallthrough operators that require 4.2, and due to not using the proper `#!/usr/bin/env bash` shebang it's incredily difficult to configure it to use another one.

### Dependencies

 - PTXDist 2025.04.0
 	- `build-essential`
	- `pkg-config`
	- `ncurses-dev`
	- `gawk`
	- `flex`
	- `bison`
	- `texinfo`
	- `unzip`
 - OSELAS.Toolchain-2024.11.1

> The version numbers of PTXDist and OSELAS.Toolchain() are important, PTXDist _must_ be matched to the version expected by OSELAS.Toolchain().

### Linux Instructions

#### Toolchain

First we need to download and install the toolchain package for our target. Penaguintronix provides pre-built toolchains for common targets such as x86-64, aarch64, risc-v, etc. so we just need to download the pre-built toolchain from the package manager. This includes the nessessarily compilers and tools for building for that target.

```bash
$ echo "deb [signed-by=/usr/share/keyrings/pengutronix-archive-keyring.gpg] http://debian.pengutronix.de/debian/ bookworm main contrib non-free" > /etc/apt/sources.list.d/pengutronix.list

# Insecure required for first run so we can install the keyring package.
# see: https://debian.pengutronix.de
$ apt -o="Acquire::AllowInsecureRepositories=true" update
$ apt-get install -y --allow-unauthenticated pengutronix-archive-keyring

# Update securely now that we have the signing key.
$ apt-get update

# To list the available toolchain packages.
$ apt-cache search "oselas.toolchain-.*-x86-64.*unknown.*"

# Requires around 500MB and will take a minute or two to download.
$ apt-get install -y oselas.toolchain-2024.11.1-x86-64-unknown-linux-gnu-gcc-14.2.1-clang-19.1.7-glibc-2.40-binutils-2.43.1-kernel-6.11.6-sanitized
# Check that the right package was installed (file will not exist otherwise)
$ ls /opt/OSELAS.Toolchain-2024.11.1/x86_64-unknown-linux-gnu/
```

#### PTXDist

Next, we need to install the PTXDist tool. PTXDist is mostly a collection of shell scripts and wrappers around common Linux cross-compilations tools to make operating with them easier.

> NOTE: Version 2025.05.0 is required, otherwise you will get an ambiguous version error when trying to perform operations in the PTXDist project.

```bash
$ wget https://public.pengutronix.de/software/ptxdist/ptxdist-2025.05.0.tar.bz2
$ tar -xjf ptxdist-2025.05.0.tar.bz2
$ cd ptxdist-2025.05.0
$ apt install build-essential pkg-config ncurses-dev gawk flex bison texinfo unzip
$ ./configure
$ make
$ sudo make install
```

## Building

Once we have the compiler toolchain and PTXDist tool installed, we can set the active set of project, platform and toolchain configs to reference the local project, and the previously installed toolchain package.

```bash
$ cd distro/
$ apt install python3.11-venv bc 
$ ptxdist select configs/ptxconfig
$ ptxdist platform configs/platform-x86_64/platformconfig
$ ptxdist toolchain /opt/OSELAS.Toolchain-2024.11.1/x86_64-unknown-linux-gnu/gcc-14.2.1-clang-19.1.7-glibc-2.40-binutils-2.43.1-kernel-6.11.6-sanitized/bin
```

Now that we have the right platform and configs selected, we can build the Linux distro!

```bash
# Build the Linux tools, programs, kernel, etc.
$ ptxdist go
# Build the images for installing the distro onto a device.
$ ptxdist images
```

Now you should have image files in `platform-x86_64/images`!

The main image file that we care about is `hd.img`, it's a partitioned image containing the EFI bootloader and kernel ready to run on a target x86 system.
