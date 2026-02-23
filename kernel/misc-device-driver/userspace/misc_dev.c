// SPDX-License-Identifier: GPL-2.0

// Copyright (C) 2024 Google LLC.

#include <stdio.h>
#include <stdlib.h>
#include <errno.h>
#include <fcntl.h>
#include <unistd.h>
#include <sys/ioctl.h>

#define RUST_MISC_DEV_FAIL _IO('|', 0)
#define RUST_MISC_DEV_HELLO _IO('|', 0x80)
#define RUST_MISC_DEV_GET_VALUE _IOR('|', 0x81, int)
#define RUST_MISC_DEV_SET_VALUE _IOW('|', 0x82, int)

int main() {
   int value, new_value;
   int fd, ret;

   // Open the device file
   printf("Opening /dev/rust-misc-device for reading and writing\n");
   fd = open("/dev/rust-misc-device", O_RDWR);
   if (fd < 0) {
     perror("open");
     return errno;
   }

   // Make call into driver to say "hello"
   printf("Calling Hello\n");
   ret = ioctl(fd, RUST_MISC_DEV_HELLO, NULL);
   if (ret < 0) {
     perror("ioctl: Failed to call into Hello");
     close(fd);
     return errno;
   }

   // Get initial value
   printf("Fetching initial value\n");
   ret = ioctl(fd, RUST_MISC_DEV_GET_VALUE, &value);
   if (ret < 0) {
     perror("ioctl: Failed to fetch the initial value");
     close(fd);
     return errno;
   }

   value++;

   // Set value to something different
   printf("Submitting new value (%d)\n", value);
   ret = ioctl(fd, RUST_MISC_DEV_SET_VALUE, &value);
   if (ret < 0) {
     perror("ioctl: Failed to submit new value");
     close(fd);
     return errno;
   }

   // Ensure new value was applied
   printf("Fetching new value\n");
   ret = ioctl(fd, RUST_MISC_DEV_GET_VALUE, &new_value);
   if (ret < 0) {
     perror("ioctl: Failed to fetch the new value");
     close(fd);
     return errno;
   }

   if (value != new_value) {
     printf("Failed: Committed and retrieved values are different (%d - %d)\n", value, new_value);
     close(fd);
     return -1;
   }

   // Call the unsuccessful ioctl
   printf("Attempting to call in to an non-existent IOCTL\n");
   ret = ioctl(fd, RUST_MISC_DEV_FAIL, NULL);
   if (ret < 0) {
     perror("ioctl: Succeeded to fail - this was expected");
   } else {
     printf("ioctl: Failed to fail\n");
     close(fd);
     return -1;
   }

   // Close the device file
   printf("Closing /dev/rust-misc-device\n");
   close(fd);

   printf("Success\n");
   return 0;
}
