use core::ops::DerefMut;
use core::{fmt::Write, ptr::NonNull};
use core::cell::SyncUnsafeCell;

pub struct Console {
    pub logger: NonNull<dyn Write + 'static>,
    pub next: Option<NonNull<Console>>
}

unsafe impl Send for Console {}

impl Console
{
    pub fn new<T>(logger: &T) -> Self    
    where T: Write + 'static
    {
        Self {
            logger: NonNull::from(logger),
            next: None,
        }
    }
    #[allow(invalid_reference_casting)]
    pub unsafe fn as_mut_static(&self) -> &'static mut Self {
        unsafe {
            let const_ptr = self as *const Self;
            let mut_ptr = const_ptr as *mut Self;
            &mut *mut_ptr
        }
    }
}

unsafe impl Sync for Console {}

pub struct ConsoleRegistry {
    pub head: Option<NonNull<Console>>
}

impl ConsoleRegistry {
    pub const fn default() -> Self {
        Self { head: None }
    }
    
    pub fn register(&mut self, console: &'static mut Console)
    {
        console.next = self.head;
        self.head = Some(NonNull::from(console));
    }

    pub unsafe fn broadcast(&mut self, args: core::fmt::Arguments)
    {
        let mut curr = self.head;
        while let Some(mut ptr) = curr {
            unsafe 
            {
                let console = ptr.as_mut();
                let _ = console.logger.as_mut().write_fmt(args);
                curr = console.next;
            }
        }
    }
}

unsafe impl Sync for ConsoleRegistry {}

pub static CONSOLE_REGISTRY: SyncUnsafeCell<ConsoleRegistry> = SyncUnsafeCell::new(ConsoleRegistry::default());

