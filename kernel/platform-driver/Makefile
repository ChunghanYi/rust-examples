# SPDX-License-Identifier: GPL-2.0

obj-m := rust_driver_platform.o

#KDIR ?= /lib/modules/$(shell uname -r)/build
KDIR=<YOUR_PATH>/linux_6.18.y/linux
PWD := $(shell pwd)

.PHONY: modules
modules:
	$(MAKE) ARCH=arm64 LLVM=1 -C $(KDIR) M=$(PWD) modules

.PHONY: clean
clean:
	$(MAKE) -C $(KDIR) M=$(PWD) clean
