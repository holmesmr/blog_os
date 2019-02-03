#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use blog_os::integration_prelude::*;

kernel_integration_test! {{
    serial_println!("ok");

    pass_integration_test();
}}

integration_fail_on_panic!();
