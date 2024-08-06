use crate::metadata::{Annotated, AnnotatedKind};

use super::AssertionContext;

#[doc(hidden)]
pub fn __annotate_assertion<T, O>(
    cx: AssertionContext,
    subject: Annotated<T>,
    next: fn(AssertionContext, T) -> O,
) -> O {
    // Build next context
    let mut next_cx = cx.next();
    next_cx.annotate(
        "received",
        match subject.kind() {
            AnnotatedKind::Debug => subject.as_str(),
            AnnotatedKind::Stringify => "? (no debug representation)",
        },
    );

    // Call inner assertion
    next(next_cx, subject.into_inner())
}
