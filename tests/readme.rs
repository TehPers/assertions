use expecters::prelude::*;

const TEST_CONTENTS: &str = include_str!("./readme_example.rs");
const README_CONTENTS: &str = include_str!("../README.md");

#[test]
fn readme_example_is_correct() {
    let example = get_section("EXAMPLE").expect("example section not found");
    expect!(example, to_start_with("```rust"));
    expect!(example, to_end_with("```"));

    // Trim out code block styling and extra attributes and normalize line
    // endings
    let example = example["```rust".len()..example.len() - "```".len()]
        .trim()
        .replace("\r\n", "\n");
    let test_contents = TEST_CONTENTS
        .splitn(3, '\n')
        .last()
        .unwrap()
        .split(r#"#[ignore = "run this test manually to see the output"]"#)
        .map(str::trim)
        .collect::<Vec<_>>()
        .join("\n")
        .replace("\r\n", "\n");
    let test_contents = test_contents.trim();

    expect!(example, to_equal(test_contents));
}

fn get_section(name: &str) -> Option<&'static str> {
    let (_, rest) = README_CONTENTS.split_once(&format!("<!-- {name} -->"))?;
    let (contents, _) = rest.split_once(&format!("<!-- /{name} -->"))?;
    Some(contents.trim())
}
