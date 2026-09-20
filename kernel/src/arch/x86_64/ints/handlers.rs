use crate::{COM1_SERIAL, arch::x86_64::ints::idt::{ATTR_INT, IDT, IDTEntry}};
use lazy_static::lazy_static;
use core::cell::SyncUnsafeCell;

// Interrupt Frame
#[repr(C, packed)]
pub struct IntFrame
{
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    r11: u64,
    r10: u64,
    r9: u64,
    r8: u64,
    rbp: u64,
    rdi: u64,
    rsi: u64,
    rdx: u64,
    rcx: u64,
    rbx: u64,
    rax: u64,

    error_code: u64,
        
    rip: u64,
    cs: u64,
    rflags: u64,
    rsp: u64,
    ss: u64
}

pub const EMPTY_TABLE: IDT = IDT::new();

// Inspired by SafaOS
macro_rules! make_handler_inner {
    ($name:path, $error_code:literal) => {{
        #[unsafe(naked)]
        extern "C" fn wrapper() -> ! {
            const _: extern "C" fn(&mut IntFrame) = $name;

            core::arch::naked_asm!(
            concat!(
                $error_code,
                "
                push rax
                push rbx
                push rcx
                push rdx
                push rsi
                push rdi
                push rbp
                push r8
                push r9
                push r10
                push r11
                push r12
                push r13
                push r14
                push r15
                
                mov rdi, rsp
                cld
    
                call {handler}
    
                pop r15
                pop r14
                pop r13
                pop r12
                pop r11
                pop r10
                pop r9
                pop r8
                pop rbp
                pop rdi
                pop rsi
                pop rdx
                pop rcx
                pop rbx
                pop rax
    
                add rsp, 8
                iretq
                "
                ), 
                handler = sym $name
            );
        }

        wrapper
    }};
}

macro_rules! make_handler {
    ($name:path, true) => {
        make_handler_inner!($name, "")
    };

    ($name:path, false) => {
        make_handler_inner!($name, "push 0\n")
    };
}

macro_rules! create_idt {
    ($(($idx:expr, $handler:expr, $attr:expr $(, $ist:literal)?)),*) => {
        {
            let mut table = EMPTY_TABLE;
            $(
                let idx: usize = $idx as usize;
                let handler: u64 = $handler as *const () as u64;
                let attr: u8 = $attr;
                let ist: u8 = {
                    let ist_val: i8 = 0;
                    $(let ist_val = $ist as i8;)?
                    ist_val as u8
                };
                table.0[idx] = IDTEntry::new(handler, attr);
                table.0[idx].ist = ist;
            )*
            SyncUnsafeCell::new(table)
        }
    };
}


extern "C" fn page_fault_handler(frame: &mut IntFrame) {
    COM1_SERIAL.puts("Page Faulted!");
    loop {}
}

lazy_static! {
    pub static ref INTERRUPT_TABLE: SyncUnsafeCell<IDT> = create_idt!(
        (0xE, make_handler!(page_fault_handler, false), ATTR_INT)
    );
}


