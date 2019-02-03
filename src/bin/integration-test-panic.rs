#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![allow(unreachable_code)]

use blog_os::integration_prelude::*;

kernel_integration_test!{{
    panic!("test panic");

    fail_integration_test(None);
}}

integration_pass_on_panic!();