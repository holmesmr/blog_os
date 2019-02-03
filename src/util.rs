#![cfg_attr(test, allow(unused_imports))]
use core::panic::PanicInfo;
use crate::println;

#[cfg(feature = "integration-test")]
#[cfg(not(test))]
pub unsafe fn exit_qemu() {
    use x86_64::instructions::port::Port;

    let mut port = Port::<u32>::new(0xf4);
    port.write(0);
}

/// This function is called on panic in the kernel.
#[cfg(not(feature = "integration-test"))]
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

/// This function is called on panic in integration tests.
#[cfg(feature = "integration-test")]
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use crate::serial_println;

    serial_println!("[integration-test-result:fail]");

    serial_println!("{}", info);

    unsafe { exit_qemu(); }
    loop {}
}

#[macro_export]
macro_rules! kernel_entrypoint {
    ($body:block) => (

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

            loop {}
        }
    )
}

#[macro_export]
macro_rules! kernel_integration_test {
    ($body:block) => (
        #[cfg(feature = "integration-test")]
        #[cfg(not(test))]
        #[inline(always)]
        fn run_entrypoint() {
            $body
        }

        #[cfg(feature = "integration-test")]
        #[cfg(not(test))]
        #[no_mangle]
        pub extern "C" fn _start() -> ! {
            run_entrypoint();

            serial_println!("[integration-test-result:pass]");

            unsafe { $crate::util::exit_qemu(); }

            loop {}
        }
    )
}