include ../make.conf 

CFLAGS := \
    -Wall \
    -Wextra \
    -std=gnu11 \
    -ffreestanding \
    -fno-stack-protector \
    -fno-stack-check \
    -fno-PIC \

CPPFLAGS  		:=  -I$(ROOT_DIR)/boot/ \
					-I$(ROOT_DIR)/kernel/include \
					-I$(ROOT_DIR)/klibc/include \
					-I$(ROOT_DIR)/libs/ \
					-MMD \
					-MP

NASMFLAGS 		:= -Wall -f elf64  

MODULE_NAME := kernel
include $(BUILD_MK)
