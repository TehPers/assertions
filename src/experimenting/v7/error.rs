// - combinators are built in the direction data flows
//   - eg. All<Not<WhenReady<Root<T>>>>
// - assertions are built in the direction the assertion is wrapped
//   - eg. Root<T, WhenReady<Not<All<SimpleAssert<T>>>>>
//   - this is opposite the direction combinators are built in
// - combinators:
//   - need to know the type of the next input to constrain the chain
//     - ex should be possible to know when `.all()` is applicable
//   - already know the input type since the combinator wraps the root value
//     - no need to be generic over it as a result
//   - making them generic over the "next assertion" at the trait level makes it
//     difficult to create bounds over the trait since a known assertion type
//     must be provided to get the associated types
//   - assertion type must be generic over the "next assertion" since it wraps
//     the assertion
// - assertions:
//   - do not need to be generic over the input type because they already know
//     the type they are being executed on
//   - have variable return types - `()`, `impl Future<Output = ()>`, etc
//   - `NotAssertion<T>` needs to know how to invert the return type between
//     success and failure
//     - can use a trait for this rather than requiring a fixed output type
#[derive(Debug, Default)]
pub struct AssertionError {
    fields: Vec<(String, String)>,
}
