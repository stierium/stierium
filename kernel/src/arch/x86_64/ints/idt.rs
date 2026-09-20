use core::arch::asm;

use lazy_static::lazy_static;
use crate::{COM1_SERIAL, arch::x86_64::ints::handlers::{INTERRUPT_TABLE}};

pub const ATTR_INT: u8 = 0x8E;
pub const ATTR_TRAP: u8 = 0x8F;

#[repr(C, packed)]
pub struct IDTR {
    sz: u16,  // size
    off: u64, // offset
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct IDTEntry {
    low_off: u16, // low offset
    ss: u16,
    pub ist: u8,
    type_n_attr: u8,
    mid_off: u16,
    high_off: u32,
    res: u32,
}

impl IDTEntry {
    pub const fn new(handler: u64, type_n_attr: u8) -> Self {
        Self {
            low_off: (handler & 0xFFFF) as u16,
            mid_off: ((handler >> 16) & 0xFFFF) as u16,
            high_off: ((handler >> 32) & 0xFFFFFFFF) as u32,
            ss: 0x08,
            type_n_attr: type_n_attr,
            ist: 0,
            res: 0,
        }
    }

    pub const fn default() -> Self {
        Self {
            low_off: 0,
            mid_off: 0,
            high_off: 0,
            ss: 0,
            type_n_attr: 0,
            ist: 0,
            res: 0,
        }
    }
}

#[repr(C, packed)]
pub struct IDT(pub [IDTEntry; 256]);

/// Represents different rings
pub enum Ring {
    /// Ring 0
    Kernel = 0,

    /// Ring 3
    User = 3,
}

/// Represents interrupt entry kinds
pub enum EntryKind {
    /// Can be issued only when interrupts are enabled.
    Trap = 0xF,
    /// Can be issued even when interrupts are disabled.
    Int = 0xE,
}

impl IDT {
    pub const fn new() -> Self {
        Self([const { IDTEntry::default() }; 256])
    }
}

lazy_static! {
    pub static ref idtr: IDTR = IDTR {
        sz: (size_of::<IDT>() - 1) as u16,
        off: INTERRUPT_TABLE.get() as u64
    };
}

pub unsafe fn idt_init()
{
    unsafe {
        asm!(
            "lidt [{}]",
            in(reg) &*idtr,
            options(readonly, nostack, preserves_flags)
        );
    }

    COM1_SERIAL.puts("IDT Initalized!\n");
}


