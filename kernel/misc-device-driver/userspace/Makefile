# SPDX-License-Identifier: GPL-2.0

CROSS     = aarch64-linux-gnu-
CC		  = $(CROSS)gcc
STRIP	  = $(CROSS)strip
MAKE	  = make
CP		  = cp
RM		  = rm
LIBS	  =
CFLAGS	  = -W -Wall -D_GNU_SOURCE -static
OFLAGS    = -W -Wall -D_GNU_SOURCE
COMMONLIB =

TARGET	= misc_dev_app 

.SUFFIXES: .c .cpp .o .O .h

.c.o:
	$(CC) $(OFLAGS) -c $< -o $@

all	:	$(TARGET)

$(TARGET):	misc_dev.o
	$(CC) $(CFLAGS)	-o $@ $^ $(COMMONLIB) $(LIBS)
strip:
	$(STRIP) $(TARGET)
clean:
	$(RM) -rf *.o $(TARGET)
