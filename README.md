I'm reading these resources:

* http://os.phil-opp.com/
* http://intermezzos.github.io/

Please open an issue if you can recommend more!

# Dependencies

An Ansible task to install the packages for Arch.

```yaml
    - name: Install unikernel packages
      package: name={{item}} state=latest
      with_items:
        - nasm
        - grub
        - libisoburn # xorriso
        - mtools # mformat
        - qemu
```


