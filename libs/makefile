include ../make.conf 

CFLAGS := \
    -Wall \
    -Wextra \
    -std=gnu11 \
    -ffreestanding \
    -fno-stack-protector \
    -fno-stack-check \
    -fno-PIC \

CPPFLAGS  		:=  -Iinclude \
					-MMD \
					-MP

NASMFLAGS 		:= -Wall -f elf64  

MODULE_NAME := libs
include $(BUILD_MK)

