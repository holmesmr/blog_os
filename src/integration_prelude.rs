pub use crate::{
    serial_print,
    serial_println,
    kernel_integration_test,
    integration_fail_on_panic,
    integration_pass_on_panic
};

#[cfg(feature = "integration-test")]
pub use crate::util::{fail_integration_test, pass_integration_test};
