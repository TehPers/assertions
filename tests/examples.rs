use expecters::prelude::*;

#[test]
#[ignore = "run this test manually to see the output"]
fn string_diff() {
    let left = "The quick\nbrown fox\njumped over\nthe lazy\ndog.";
    let right = "the quick brown\nspotted fox\njumped over\nthe lazyish\ndog.";
    expect!(left, to_equal(right));
}

#[test]
#[ignore = "run this test manually to see the output"]
fn debug_diff() {
    #[derive(PartialEq, Eq, Debug)]
    struct A {
        inner: i32,
    }

    impl A {
        pub fn new(inner: i32) -> Self {
            Self { inner }
        }
    }

    let subject = vec![A::new(1), A::new(3), A::new(3), A::new(512), A::new(761)];
    let expected = vec![A::new(1), A::new(2), A::new(3), A::new(513), A::new(761)];

    expect!(subject, to_equal(expected));
}
