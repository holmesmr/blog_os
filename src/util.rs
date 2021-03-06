#![cfg_attr(test, allow(unused_imports))]
use core::panic::PanicInfo;

#[cfg(feature = "integration-test")]
#[cfg(not(test))]
pub unsafe fn exit_qemu() {
    use x86_64::instructions::port::Port;

    let mut port = Port::<u32>::new(0xf4);
    port.write(0);
}

#[inline(always)]
pub fn _halt_impl() -> ! {
    use x86_64::instructions::hlt;

    loop {
        hlt()
    }
}

#[inline(always)]
#[cfg(feature = "integration-test")]
pub fn halt() -> ! {
    unsafe {
        exit_qemu();
    }

    _halt_impl()
}

#[inline(always)]
#[cfg(not(feature = "integration-test"))]
pub fn halt() -> ! {
    _halt_impl()
}

/// This function is called on panic in the kernel.
#[cfg(not(feature = "integration-test"))]
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use crate::println;

    println!("{}", info);
    halt()
}

/// This function is called on panic in integration tests.
#[cfg(feature = "integration-test")]
#[cfg(not(test))]
pub fn fail_integration_test(info: Option<&PanicInfo>) {
    use crate::serial_println;

    serial_println!("[integration-test-result:fail]");

    if let Some(info) = info {
        serial_println!("{}", info);
    }

    unsafe {
        exit_qemu();
    }
}

/// This function is called on panic in integration tests.
#[cfg(feature = "integration-test")]
#[cfg(not(test))]
pub fn pass_integration_test() {
    use crate::serial_println;

    serial_println!("[integration-test-result:pass]");
}

#[macro_export]
macro_rules! kernel_entrypoint {
    ($body:block) => {
        #[cfg(not(feature = "integration-test"))]
        #[cfg(not(test))]
        #[inline(always)]
        fn run_entrypoint() {
            $body
        }

        #[cfg(not(feature = "integration-test"))]
        #[cfg(not(test))]
        #[no_mangle]
        pub extern "C" fn _start() -> ! {
            run_entrypoint();

            $crate::util::halt()
        }
    };
}

#[macro_export]
#[cfg(feature = "integration-test")]
macro_rules! kernel_integration_test {
    ($body:block) => {
        #[cfg(not(test))]
        #[inline(always)]
        fn run_entrypoint() {
            $body
        }

        #[cfg(not(test))]
        #[no_mangle]
        pub extern "C" fn _start() -> ! {
            run_entrypoint();

            $crate::util::halt()
        }
    };
}

#[macro_export]
#[cfg(feature = "integration-test")]
macro_rules! integration_fail_on_panic {
    () => {
        use core::panic::PanicInfo;

        #[cfg(not(test))]
        #[panic_handler]
        fn panic(info: &PanicInfo) -> ! {
            fail_integration_test(core::option::Option::Some(info));

            $crate::util::halt()
        }
    };
}

#[macro_export]
#[cfg(feature = "integration-test")]
macro_rules! integration_pass_on_panic {
    () => {
        use core::panic::PanicInfo;

        #[cfg(not(test))]
        #[panic_handler]
        fn panic(info: &PanicInfo) -> ! {
            serial_println!("[integration-test-result:pass]");

            $crate::util::halt()
        }
    };
}

#[macro_export]
#[cfg(not(feature = "integration-test"))]
macro_rules! kernel_integration_test {
    ($_body:block) => {};
}

#[macro_export]
#[cfg(not(feature = "integration-test"))]
macro_rules! integration_fail_on_panic {
    () => {};
}

#[macro_export]
#[cfg(not(feature = "integration-test"))]
macro_rules! integration_pass_on_panic {
    () => {};
}
