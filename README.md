# TODO: name

Build complex, self-describing assertions by chaining together reusable methods.
Supports both synchronous and asynchronous assertions.

## Example

<!-- EXAMPLE -->
```rust
use expecters::prelude::*;

#[tokio::test]
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
```
<!-- /EXAMPLE -->

Error message:

```text
thread 'test' panicked at README.md:13:6:
assertion failed:
  at: README.md:8:5 [readme_example]
  subject: [get_cat_url(0), get_cat_url(40), get_cat_url(42)]

steps:
  all:
    received: ? (no debug representation)
    index: 2

  when_ready:
    received: ? (no debug representation)

  to_end_with: substring not found
    received: "cats/42.png"
    expected: "0.png"
```

Supports colored messages with the `colors` feature.

## Built-in assertions

### General

| Assertion                        | Description        |
| -------------------------------- | ------------------ |
| `to_equal`                       | x == y             |
| `to_equal_approximately`         | \|x - y\| < d      |
| `to_be_greater_than`             | x > y              |
| `to_be_greater_than_or_equal_to` | x >= y             |
| `to_be_less_than`                | x < y              |
| `to_be_less_than_or_equal_to`    | x <= y             |
| `to_be_one_of`                   | x in [y1, y2, ...] |
| `to_satisfy`                     | f(x) -> true       |
| `to_satisfy_with`                | f(x) -> Ok         |

| Modifier | Description    |
| -------- | -------------- |
| `not`    | negates result |
| `map`    | maps subject   |

### Options

| Assertion    | Description |
| ------------ | ----------- |
| `to_be_some` | x is Some   |
| `to_be_none` | x is None   |

| Modifier         | Description   |
| ---------------- | ------------- |
| `to_be_some_and` | extracts Some |

### Results

| Assertion   | Description |
| ----------- | ----------- |
| `to_be_ok`  | x is Ok     |
| `to_be_err` | x is Err    |

| Modifier        | Description  |
| --------------- | ------------ |
| `to_be_ok_and`  | extracts Ok  |
| `to_be_err_and` | extracts Err |

### Strings

| Assertion           | Description       | Requires feature |
| ------------------- | ----------------- | ---------------- |
| `to_contain_substr` | x contains y      |                  |
| `to_start_with`     | x starts with y   |                  |
| `to_end_with`       | x ends with y     |                  |
| `to_match_regex`    | x matches pattern | `regex`          |

| Modifier     | Description                             |
| ------------ | --------------------------------------- |
| `chars`      | map subject to `char` sequence          |
| `as_debug`   | map subject to `Debug` representation   |
| `as_display` | map subject to `Display` representation |

### Iterators

| Assertion            | Description                  |
| -------------------- | ---------------------------- |
| `to_contain`         | x contains y                 |
| `to_contain_exactly` | x is sequentially equal to y |

| Modifier  | Description                           |
| --------- | ------------------------------------- |
| `all`     | each item satisfies assertion         |
| `any`     | at least one item satisfies assertion |
| `count`   | counts items                          |
| `nth`     | gets nth item                         |
| `as_utf8` | parses as utf8                        |

### Readers

| Modifier          | Description                           | Requires feature |
| ----------------- | ------------------------------------- | ---------------- |
| `when_read`       | reads into byte buffer                |                  |
| `when_read_async` | asynchronously reads into byte buffer | `futures`        |

### Futures

| Modifier            | Description                          | Requires feature |
| ------------------- | ------------------------------------ | ---------------- |
| `when_ready`        | gets output                          | `futures`        |
| `when_ready_before` | gets output if it completes before y | `futures`        |
| `when_ready_after`  | gets output if it completes after y  | `futures`        |

## License

This repository is dual licensed under [MIT](./LICENSE-MIT) and
[APACHE-2.0](./LICENSE-APACHE). You may choose which license you wish to use.
