pub use crate::{
    integration_fail_on_panic, integration_pass_on_panic, kernel_integration_test, serial_print,
    serial_println,
};

#[cfg(feature = "integration-test")]
pub use crate::util::{fail_integration_test, pass_integration_test};
