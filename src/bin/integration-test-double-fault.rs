#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![feature(abi_x86_interrupt)]

use blog_os::gdt;
use blog_os::integration_prelude::*;

use lazy_static::lazy_static;
use x86_64::structures::idt::{ExceptionStackFrame, InterruptDescriptorTable};

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

#[cfg(feature = "integration-test")]
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut ExceptionStackFrame,
    _error_code: u64,
) {
    use blog_os::util::halt;

    // Pass if the double fault gets handled successfully
    pass_integration_test();
    halt();
}

// Stub for normal builds
#[cfg(not(feature = "integration-test"))]
extern "x86-interrupt" fn double_fault_handler(
    _stack_frame: &mut ExceptionStackFrame,
    _error_code: u64,
) {
}

pub fn init_idt() {
    TEST_IDT.load();
}

kernel_integration_test! {{
    blog_os::gdt::init();
    init_idt();

    // Attempt to trigger a triple fault by smashing the kernel's stack
    fn stack_overflow() {
        stack_overflow(); // for each recursion, the return address is pushed
    }

    stack_overflow();

    fail_integration_test(None);
}}

integration_fail_on_panic!();
