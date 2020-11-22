use std::error;

use static_assertions::assert_impl_all;

use heim_process::ProcessError;

#[test]
fn test_public_api_contract() {
    assert_impl_all!(ProcessError: Send, Sync, error::Error);
}
