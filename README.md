# Expecters

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

Supports:

- colored messages with the `colors` feature
- diffing with the `diff` feature
- async assertions with the `futures` feature
- regular expressions with the `regex` feature

### Output diffs

Check how outputs differ based on their `Display` representations where
available:

```text
---- string_diff stdout ----
thread 'string_diff' panicked at tests\examples.rs:8:5:
assertion failed:
  at: tests\examples.rs:8:5 [examples]
  subject: "The quick\nbrown fox\njumped over\nthe lazy\ndog."

steps:
  to_equal: [1] values not equal
    received: "The quick\nbrown fox\njumped over\nthe lazy\ndog."
    expected: "the quick brown\nspotted fox\njumped over\nthe lazyish\ndog."

----- diff [1] -----
- the quick brown
+ The quick
- spotted fox
+ brown fox
  jumped over
- the lazyish
+ the lazy
  dog.
```

Or, for types that only implement `Debug`, use that representation automatically
instead:

```text
---- debug_diff stdout ----
thread 'debug_diff' panicked at tests\examples.rs:28:5:
assertion failed:
  at: tests\examples.rs:28:5 [examples]
  subject: [A { inner: 1 }, A { inner: 3 }, A { inner: 3 }, A { inner: 512 }, A { inner: 761 }]

steps:
  to_equal: [1] values not equal
    received: [A { inner: 1 }, A { inner: 3 }, A { inner: 3 }, A { inner: 512 }, A { inner: 761 }]
    expected: [A { inner: 1 }, A { inner: 2 }, A { inner: 3 }, A { inner: 513 }, A { inner: 761 }]

----- diff [1] -----
  [
      A {
          inner: 1,
      },
      A {
-         inner: 2,
+         inner: 3,
      },
      A {
          inner: 3,
      },
      A {
-         inner: 513,
+         inner: 512,
      },
      A {
          inner: 761,
      },
  ]
```

For colored output, try running some of the example tests in the `tests/`
directory.

## License

This repository is dual licensed under [MIT](./LICENSE-MIT) and
[APACHE-2.0](./LICENSE-APACHE). You may choose which license you wish to use.
