use async_once::AsyncOnce;
use lazy_static::lazy_static;
use reqwest_retry::policies::ExponentialBackoff;
use serial_test::serial;

use crate::{
    client::IBClientPortal,
    model::session::AuthStatus,
    test::utils::{get_test_account, TEST_HOST},
};

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_tickle() {
    let ib_cp_client = IBClientPortal::new(
        get_test_account(),
        TEST_HOST.to_owned(),
        false,
        ExponentialBackoff::builder().build_with_max_retries(3),
    );
    let response_result = ib_cp_client.tickle().await;
    assert!(response_result.is_ok());
    let response = response_result.unwrap();
    assert!(response.session.len() > 0);
    assert!(response.user_id > 0);
}
