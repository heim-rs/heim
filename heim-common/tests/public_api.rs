use std::error;

use static_assertions::assert_impl_all;

use heim_common::Error;

#[test]
fn test_public_api_contract() {
    assert_impl_all!(Error: Send, Sync, error::Error);
}
