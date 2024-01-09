## nwcc

The Second Futamura Projection holds that a sufficiently advanced partial evaluator, by specializing another instance of itself to an interpreter, can generate a compiler from the interpreted language.

This is... not that.

### Features

* Full Rust type checking via delegated authority to host compiler
* Compiles in a single pass
* Caches object code for extreme speed
* Rejects compilation of all known programs containing bugs

### Targets

##### Tier 1:

* aarch64-unknown-linux-gnu
* i686-pc-windows-gnu
* i686-pc-windows-msvc
* i686-unknown-linux-gnu
* x86_64-apple-darwin
* x86_64-pc-windows-gnu
* x86_64-pc-windows-msvc
* x86_64-unknown-linux-gnu

##### Tier 2:

- aarch64-apple-darwin
- aarch64-pc-windows-msvc
- aarch64-unknown-linux-musl
- arm-unknown-linux-gnueabi
- arm-unknown-linux-gnueabihf
- armv7-unknown-linux-gnueabihf
- loongarch64-unknown-linux-gnu
- powerpc-unknown-linux-gnu
- powerpc64-unknown-linux-gnu
- powerpc64le-unknown-linux-gnu
- riscv64gc-unknown-linux-gnu
- s390x-unknown-linux-gnu
- x86_64-unknown-freebsd
- x86_64-unknown-illumos
- x86_64-unknown-linux-musl
- x86_64-unknown-netbsd
- aarch64-apple-ios
- aarch64-apple-ios-sim
- aarch64-fuchsia
- aarch64-unknown-fuchsia
- aarch64-linux-android
- aarch64-unknown-none-softfloat
- aarch64-unknown-none
- aarch64-unknown-uefi
- arm-linux-androideabi
- arm-unknown-linux-musleabi
- arm-unknown-linux-musleabihf
- armebv7r-none-eabi
- armebv7r-none-eabihf
- armv5te-unknown-linux-gnueabi
- armv5te-unknown-linux-musleabi
- armv7-linux-androideabi
- armv7-unknown-linux-gnueabi
- armv7-unknown-linux-musleabi
- armv7-unknown-linux-musleabihf
- armv7a-none-eabi
- armv7r-none-eabi
- armv7r-none-eabihf
- i586-pc-windows-msvc
- i586-unknown-linux-gnu
- i586-unknown-linux-musl
- i686-linux-android
- i686-unknown-freebsd
- i686-unknown-linux-musl
- i686-unknown-uefi
- loongarch64-unknown-none
- loongarch64-unknown-none-softfloat
- nvptx64-nvidia-cuda
- riscv32imac-unknown-none-elf
- riscv32i-unknown-none-elf
- riscv32im-unknown-none-elf
- riscv32imc-unknown-none-elf
- riscv32imafc-unknown-none-elf
- riscv64gc-unknown-none-elf
- riscv64imac-unknown-none-elf
- sparc64-unknown-linux-gnu
- sparcv9-sun-solaris
- thumbv6m-none-eabi
- thumbv7em-none-eabi
- thumbv7em-none-eabihf
- thumbv7m-none-eabi
- thumbv7neon-linux-androideabi
- thumbv7neon-unknown-linux-gnueabihf
- thumbv8m.base-none-eabi
- thumbv8m.main-none-eabi
- thumbv8m.main-none-eabihf
- wasm32-unknown-emscripten
- wasm32-unknown-unknown
- wasm32-wasi
- wasm32-wasi-preview1-threads
- x86_64-apple-ios
- x86_64-fortanix-unknown-sgx
- x86_64-fuchsia
- x86_64-unknown-fuchsia
- x86_64-linux-android
- x86_64-pc-solaris
- x86_64-unknown-linux-gnux32
- x86_64-unknown-none
- x86_64-unknown-redox
- x86_64-unknown-uefi

##### Tier 3:

- arm64e-apple-ios
- arm64e-apple-darwin
- aarch64-apple-ios-macabi
- aarch64-apple-tvos
- aarch64-apple-tvos-sim
- aarch64-apple-watchos
- aarch64-apple-watchos-sim
- aarch64-kmc-solid_asp3
- aarch64-nintendo-switch-freestanding
- aarch64-pc-windows-gnullvm
- aarch64-unknown-linux-ohos
- aarch64-unknown-teeos
- aarch64-unknown-nto-qnx710
- aarch64-unknown-freebsd
- aarch64-unknown-hermit
- aarch64-unknown-illumos
- aarch64-unknown-linux-gnu_ilp32
- aarch64-unknown-netbsd
- aarch64-unknown-openbsd
- aarch64-unknown-redox
- aarch64-uwp-windows-msvc
- aarch64-wrs-vxworks
- aarch64_be-unknown-linux-gnu_ilp32
- aarch64_be-unknown-linux-gnu
- aarch64_be-unknown-netbsd
- arm64_32-apple-watchos
- armeb-unknown-linux-gnueabi
- armv4t-none-eabi
- armv4t-unknown-linux-gnueabi
- armv5te-none-eabi
- armv5te-unknown-linux-uclibceabi
- armv6-unknown-freebsd
- armv6-unknown-netbsd-eabihf
- armv6k-nintendo-3ds
- armv7-sony-vita-newlibeabihf
- armv7-unknown-linux-ohos
- armv7-unknown-linux-uclibceabi
- armv7-unknown-linux-uclibceabihf
- armv7-unknown-freebsd
- armv7-unknown-netbsd-eabihf
- armv7-wrs-vxworks-eabihf
- armv7a-kmc-solid_asp3-eabi
- armv7a-kmc-solid_asp3-eabihf
- armv7a-none-eabihf
- armv7k-apple-watchos
- armv7s-apple-ios
- avr-unknown-gnu-atmega328
- bpfeb-unknown-none
- bpfel-unknown-none
- csky-unknown-linux-gnuabiv2
- csky-unknown-linux-gnuabiv2hf
- hexagon-unknown-none-elf
- hexagon-unknown-linux-musl
- i386-apple-ios
- i586-pc-nto-qnx700
- i586-unknown-netbsd
- i686-apple-darwin
- i686-pc-windows-msvc
- i686-pc-windows-gnullvm
- i686-unknown-haiku
- i686-unknown-hurd-gnu
- i686-unknown-netbsd
- i686-unknown-openbsd
- i686-uwp-windows-gnu
- i686-uwp-windows-msvc
- i686-win7-windows-msvc
- i686-wrs-vxworks
- m68k-unknown-linux-gnu
- mips-unknown-linux-gnu
- mips-unknown-linux-musl
- mips-unknown-linux-uclibc
- mips64-openwrt-linux-musl
- mips64-unknown-linux-gnuabi64
- mips64-unknown-linux-muslabi64
- mips64el-unknown-linux-gnuabi64
- mips64el-unknown-linux-muslabi64
- mipsel-unknown-linux-gnu
- mipsel-unknown-linux-musl
- mipsel-unknown-netbsd
- mipsel-sony-psp
- mipsel-sony-psx
- mipsel-unknown-linux-uclibc
- mipsel-unknown-none
- mipsisa32r6-unknown-linux-gnu
- mipsisa32r6el-unknown-linux-gnu
- mipsisa64r6-unknown-linux-gnuabi64
- mipsisa64r6el-unknown-linux-gnuabi64
- msp430-none-elf
- powerpc-unknown-linux-gnuspe
- powerpc-unknown-linux-musl
- powerpc-unknown-netbsd
- powerpc-unknown-openbsd
- powerpc-wrs-vxworks-spe
- powerpc-wrs-vxworks
- powerpc64-unknown-freebsd
- powerpc64le-unknown-freebsd
- powerpc-unknown-freebsd
- powerpc64-unknown-linux-musl
- powerpc64-wrs-vxworks
- powerpc64le-unknown-linux-musl
- powerpc64-unknown-openbsd
- powerpc64-ibm-aix
- riscv32gc-unknown-linux-gnu
- riscv32gc-unknown-linux-musl
- riscv32imac-unknown-xous-elf


### Usage

nwcc -o a.out src/program.rs