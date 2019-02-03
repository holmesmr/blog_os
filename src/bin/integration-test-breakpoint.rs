#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use blog_os::integration_prelude::*;

kernel_integration_test! {{
    blog_os::interrupts::init_idt();

    x86_64::instructions::int3();

    pass_integration_test();
}}

integration_fail_on_panic!();
