# SPDX-License-Identifier: GPL-2.0

obj-m := beagleplay-user-button.o

#KDIR ?= /lib/modules/$(shell uname -r)/build
KDIR = <YOUR_KERNEL_PATH>/linux
PWD := $(shell pwd)

.PHONY: modules
modules:
	$(MAKE) ARCH=arm64 LLVM=1 -C $(KDIR) M=$(PWD) modules

.PHONY: clean
clean:
	$(MAKE) -C $(KDIR) M=$(PWD) clean
