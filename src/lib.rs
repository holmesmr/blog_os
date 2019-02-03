#![cfg_attr(not(test), no_std)]
#![feature(abi_x86_interrupt)]

pub mod gdt;
pub mod integration_prelude;
pub mod interrupts;
pub mod prelude;
pub mod serial;
pub mod util;
pub mod vga_buffer;
