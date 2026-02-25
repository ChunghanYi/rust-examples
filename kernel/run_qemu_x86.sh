#!/bin/sh

/your_path/qemu-system-x86_64 -nographic -kernel /your_path/bzImage -append "console=ttyS0" -m 2048 -initrd /your_path/initramfs.cpio.gz -virtfs local,path=/your_path/modules,security_model=none,mount_tag=modules -acpitable file=/your_path/ssdt.aml
