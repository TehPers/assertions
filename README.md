# TODO: name

Build complex, self-describing assertions by chaining together reusable methods.
Supports both synchronous and asynchronous assertions.

## Example

```rust
use expecters::prelude::*;

#[tokio::test]
async fn test() {
    expect!(1, as_display, to_equal("1"));
    expect!(1..=5, count, to_equal(5));

    expect!(get_cat_url(0), when_ready, to_contain_substr(".png")).await;
}

async fn get_cat_url(id: u32) -> String {
    format!("cats/{id}.png")
}
```
