use expecters::prelude::*;

#[test]
fn not_debug() {
    #[derive(Clone, PartialEq)]
    struct NotDebug<T>(T);

    expect!(NotDebug(1), to_equal(NotDebug(1)));
    expect!([NotDebug(1)], all, to_equal(NotDebug(1)));
}
