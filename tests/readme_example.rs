#![cfg(feature = "futures")]

use expecters::prelude::*;

#[tokio::test]
#[ignore]
async fn test() {
    expect!(1, as_display, to_equal("1"));
    expect!(1..=5, count, to_equal(5));

    expect!(
        [get_cat_url(0), get_cat_url(40), get_cat_url(42)],
        all,
        when_ready,
        to_end_with("0.png"),
    )
    .await;
}

async fn get_cat_url(id: u32) -> String {
    format!("cats/{id}.png")
}
