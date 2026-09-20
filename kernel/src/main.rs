#![no_std]
#![no_main]
#![feature(sync_unsafe_cell)]

pub mod arch;
pub mod console;

use core::arch::asm;
use core::cell::SyncUnsafeCell;
use core::fmt::Write;

use lazy_static::lazy_static;
use limine::BaseRevision;
use limine::request::{FramebufferRequest, RequestsEndMarker, RequestsStartMarker};

use crate::arch::x86_64::gdt::gdt_init;
use crate::arch::x86_64::ints::idt::idt_init;
use crate::arch::x86_64::serial::Serial;
use crate::console::{CONSOLE_REGISTRY, Console};

#[used]
#[unsafe(link_section = ".requests")]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".requests")]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[unsafe(link_section = ".requests_start_marker")]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();
#[used]
#[unsafe(link_section = ".requests_end_marker")]
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

pub static COM1_SERIAL: Serial = Serial::new(0x3F8);

lazy_static! 
{
    pub static ref COM1_CONSOLE: Console = Console::new::<Serial>(&COM1_SERIAL);
}


#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    assert!(BASE_REVISION.is_supported());

    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
            for i in 0..100_u64 {
                let pixel_offset = i * framebuffer.pitch() + i * 4;

                unsafe {
                    framebuffer
                        .addr()
                        .add(pixel_offset as usize)
                        .cast::<u32>()
                        .write(0xFFFFFFFF)
                };
            }
        }
    }

    COM1_SERIAL.init();
    
    unsafe {
        gdt_init();
        idt_init();
    };

    unsafe {
        (*CONSOLE_REGISTRY.get()).register(COM1_CONSOLE.as_mut_static());
        (*CONSOLE_REGISTRY.get()).broadcast(format_args!("Hello, World!\n"));
    }

    unsafe {
        let ptr = 0x0usize as *mut u64;
        core::ptr::read_volatile(ptr);        
    }
    
    hcf();
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    hcf();
}

fn hcf() -> ! {
    loop {
        unsafe {
            #[cfg(target_arch = "x86_64")]
            asm!("hlt");
            #[cfg(any(target_arch = "aarch64"))]
            asm!("wfi");
        }
    }
}
