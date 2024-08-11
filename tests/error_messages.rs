use expecters::prelude::*;

#[test]
fn simple() {
    expect!(
        try_expect!(1, not, to_equal(1)),
        to_be_err_and,
        as_display,
        to_satisfy_with(|message| {
            try_expect!(&message, to_contain_substr("subject: 1"))?;
            try_expect!(&message, to_contain_substr("not"))?;
            try_expect!(&message, to_contain_substr("to_equal"))?;
            try_expect!(&message, to_contain_substr("received: 1"))?;
            Ok(())
        }),
    );
}

#[test]
fn alias() {
    use expecters::prelude::not as my_not;

    expect!(
        try_expect!(1, my_not, to_equal(1)),
        to_be_err_and,
        as_display,
        to_contain_substr("my_not"),
    );
}

#[test]
fn non_debug() {
    #[derive(PartialEq)]
    struct NotDebug<T>(T);

    expect!(
        try_expect!(NotDebug(1), to_equal(NotDebug(2))),
        to_be_err_and,
        as_display,
        to_satisfy_with(|message| {
            try_expect!(&message, to_contain_substr("subject: NotDebug(1)"))?;
            try_expect!(&message, to_contain_substr("expected: NotDebug(2)"))?;
            Ok(())
        }),
    );
}

#[test]
fn propagated_value() {
    expect!(
        try_expect!([1, 1], all, not, to_equal(1)),
        to_be_err_and,
        as_display,
        to_contain_substr("[1, 1]"),
    );
}

#[test]
fn annotated_strings() {
    expect!(
        try_expect!("test", to_equal("")),
        to_be_err_and,
        as_display,
        to_satisfy_with(|message| {
            try_expect!(&message, to_contain_substr("\"test\""))?;
            try_expect!(&message, to_contain_substr("\"\""))?;
            Ok(())
        }),
    );
}
