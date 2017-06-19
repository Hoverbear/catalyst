# `catalyst`

I'm trying to make a unikernel. Mostly in Rust, with a bit of assembly.

It's going to be 64 bit, and intended to be run in a VM.

# Ideas

Things that might be fun to do:

* Enable SSE/SIMD.
* Teach it to network.
* Teach it to speak SSH ([Trussh](https://pijul.org/thrussh/)?)
* Deploy it somewhere.

# Resources

I'm reading these resources as I go:

* http://os.phil-opp.com/
* http://intermezzos.github.io/

Please open an issue if you can recommend more!

# Dependencies

An Ansible task to install the packages for Arch.

```yaml
    - name: Install unikernel packages
      package: name={{item}} state=latest
      with_items:
        - rustup
        - nasm
        - grub
        - libisoburn # xorriso
        - mtools # mformat
        - qemu
```

You'll also need to do:

```bash
cargo install xargo
cargo component add rust-src
# Only in this project.
rustup override set nightly
```

# Running

Yes it uses a `Makefile` right now I'm sorry.

```bash
# Build a volume which, when booted, displays glorious error messages.
make iso
# Spawn qemu and display some glorious error messages.
make run
```