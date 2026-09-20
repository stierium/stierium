use core::{arch::asm, cell::SyncUnsafeCell};

use crate::COM1_SERIAL;

#[repr(C, packed)]
struct GDTEntry {
    low_limit: u16,
    low_base: u16,
    mid_base: u8,
    access_byte: u8,
    limit_and_flags: u8,
    high_base: u8,
}

#[repr(C, packed)]
struct GDTStruct {
    null_descriptor: GDTEntry,
    kernel_code_segment: GDTEntry,
    kernel_data_segment: GDTEntry,
    user_code_segment: GDTEntry,
    user_data_segment: GDTEntry,
}

#[repr(C, packed)]
struct GDTRStruct {
    limit: u16,
    base: *mut GDTStruct,
}

static GDTR: GDTRStruct = GDTRStruct {
    limit: (size_of::<GDTStruct>() - 1) as u16,
    base: GDT.get(),
};

unsafe impl Sync for GDTRStruct {}

static GDT: SyncUnsafeCell<GDTStruct> = SyncUnsafeCell::new(GDTStruct {
    null_descriptor: GDTEntry {
        low_limit: 0,
        low_base: 0,
        mid_base: 0,
        access_byte: 0,
        limit_and_flags: 0,
        high_base: 0,
    },
    kernel_code_segment: GDTEntry {
        low_limit: 0xFFFF,
        low_base: 0,
        mid_base: 0,
        access_byte: 0x98,
        limit_and_flags: 0x20,
        high_base: 0,
    },
    kernel_data_segment: GDTEntry {
        low_limit: 0xFFFF,
        low_base: 0,
        mid_base: 0,
        access_byte: 0x92,
        limit_and_flags: 0x20,
        high_base: 0,
    },
    user_code_segment: GDTEntry {
        low_limit: 0xFFFF,
        low_base: 0,
        mid_base: 0,
        access_byte: 0xF8,
        limit_and_flags: 0x20,
        high_base: 0,
    },
    user_data_segment: GDTEntry {
        low_limit: 0xFFFF,
        low_base: 0,
        mid_base: 0,
        access_byte: 0xF2,
        limit_and_flags: 0x20,
        high_base: 0,
    },
});

pub unsafe fn gdt_init() {
    unsafe {
        asm!("lgdt [{}]", in(reg) &GDTR);

        asm!(
            "
            mov ax, 0x10
            mov gs, ax
            mov fs, ax
            mov ds, ax
            mov es, ax
            mov ss, ax
        "
        );

        asm!(
            "
            push 0x08
            lea rax, [rip + 2f]
            push rax
            retfq
            2:
            ",
            options(nostack),
        );
    }

    COM1_SERIAL.puts("GDT Initialized!\n");
}
