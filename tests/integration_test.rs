// Integration test for SigmaOS-distribution-identity-manager
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use distribution_identity_manager as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
