#!/bin/sh

/your_path/qemu-system-aarch64 -M virt -cpu cortex-a53 -nographic -kernel /your_path/Image -append "console=ttyAMA0" -m 2048 -initrd /your_path/initramfs.cpio.gz -virtfs local,path=/your_path/modules,security_model=none,mount_tag=modules
