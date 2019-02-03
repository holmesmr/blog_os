#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use blog_os::prelude::*;

kernel_entrypoint!{{
    println!("Hello World{}", "!");
}}