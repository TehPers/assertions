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

    expect!(
        [get_cat_url(0), get_cat_url(5), get_cat_url(42)],
        all,
        when_ready,
        to_end_with(".png"),
    ).await;
}

async fn get_cat_url(id: u32) -> String {
    format!("cats/{id}.png")
}
```

## License

This repository is dual licensed under [MIT](./LICENSE-MIT) and
[APACHE-2.0](./LICENSE-APACHE). You may choose which license you wish to use.
