#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use blog_os::prelude::*;

kernel_entrypoint! {{
    use blog_os::interrupts::PICS;

    println!("Hello World{}", "!");

    blog_os::gdt::init();
    blog_os::interrupts::init_idt();

    unsafe { PICS.lock().initialize() };

    println!("End of program. Halting.");
}}
