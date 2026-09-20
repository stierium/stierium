
use core::fmt::{self, Write};

use super::io::{inb, outb};

pub struct Serial
{
    port: u16,
    // maybe irq number too?
}

impl Serial
{
    const DATA_PORT: u16 = 0;
    const FIFO_COMMAND_PORT: u16 = 2;
    const LINE_COMMAND_PORT: u16 = 3;
    const MODEM_COMMAND_PORT: u16 = 4;
    const LINE_STATUS_PORT: u16 = 5;
    const LINE_ENABLE_DLAB: u8 = 0x80;
    
    pub const fn new(port: u16) -> Self {
        Self {
            port
        }
    } 

    pub fn init(&self)
    {
        unsafe {

            outb(self.port + Self::DATA_PORT + 1, 0x00);
            outb(self.port + Self::LINE_COMMAND_PORT, Self::LINE_ENABLE_DLAB);
            outb(self.port + Self::DATA_PORT, 0x03);
            outb(self.port + Self::DATA_PORT + 1, 0x00);

            outb(self.port + Self::LINE_COMMAND_PORT, 0x3);
            outb(self.port + Self::FIFO_COMMAND_PORT, 0xC7);
            outb(self.port + Self::MODEM_COMMAND_PORT, 0x0B);
            outb(self.port + Self::MODEM_COMMAND_PORT, 0x1E);

            outb(self.port + Self::DATA_PORT, 0xAE);

            outb(self.port + Self::MODEM_COMMAND_PORT, 0x0F);
        }
    }

    fn is_transmit_fifo_empty(&self) -> bool {
        unsafe { 
            (inb(self.port + Self::LINE_STATUS_PORT) & 0x20) != 0 
        }
    }
    
    pub fn putc(&self, byte: u8)
    {
        // Wait for the FIFO buffer to be empty
        while !self.is_transmit_fifo_empty() {}
        unsafe {
            if byte == b'\n' {
                outb(self.port + Self::DATA_PORT, b'\r');
            }
            outb(self.port + Self::DATA_PORT, byte);
        }
    }

    pub fn puts(&self, str: &str)
    {
        for byte in str.bytes()
        {
            self.putc(byte);
        }
    }
}

impl Write for Serial
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.puts(s);
        Ok(())
    }
}


